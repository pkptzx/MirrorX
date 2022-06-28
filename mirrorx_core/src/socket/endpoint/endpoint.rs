use super::{
    ffi::create_callback_fn,
    handler::{handle_media_transmission, handle_start_media_transmission_request},
    message::{
        EndPointMessage, EndPointMessagePacket, EndPointMessagePacketType, MediaFrame,
        StartMediaTransmissionRequest, StartMediaTransmissionResponse,
    },
};
use crate::{
    error::MirrorXError,
    media::{desktop_duplicator::DesktopDuplicator, video_encoder::VideoEncoder},
    utility::{
        nonce_value::NonceValue, serializer::BINCODE_SERIALIZER, tokio_runtime::TOKIO_RUNTIME,
    },
};
use anyhow::anyhow;
use bincode::Options;
use bytes::Bytes;
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use once_cell::sync::{Lazy, OnceCell};
use ring::aead::{OpeningKey, SealingKey};
use scopeguard::defer;
use std::{
    os::raw::c_void,
    sync::atomic::{AtomicU16, Ordering},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::{Receiver, Sender},
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{error, info, warn};

const CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub static ENDPOINTS: Lazy<DashMap<String, EndPoint>> = Lazy::new(|| DashMap::new());

macro_rules! make_endpoint_call {
    ($name:tt, $req_type:ident, $req_message_type:path, $resp_type:ident, $resp_message_type:path) => {
        pub async fn $name(&self, req: $req_type) -> Result<$resp_type, MirrorXError> {
            let reply = self.call($req_message_type(req), CALL_TIMEOUT).await?;

            if let $resp_message_type(message) = reply {
                Ok(message)
            } else {
                Err(MirrorXError::EndPointError(self.remote_device_id.clone()))
            }
        }
    };
}

macro_rules! handle_endpoint_call {
    ($remote_device_id:expr, $call_id:expr, $req:tt, $resp_type:path, $handler:tt) => {{
        TOKIO_RUNTIME.spawn(async move {
            if let Some(call_id) = $call_id {
                if let Some(endpoint) = ENDPOINTS.get(&$remote_device_id) {
                    let resp_message = match $handler(endpoint.value(), $req).await {
                        Ok(resp) => $resp_type(resp),
                        Err(err) => {
                            error!(err=?err,"handle_endpoint_call returns error");
                            EndPointMessage::Error
                        }
                    };

                    if let Err(err) = endpoint.reply(call_id,resp_message).await{
                        error!(err=?err,remote_device_id=?$remote_device_id,"handle_message: reply message failed");
                    }
                }else{
                    error!(remote_device_id=?$remote_device_id,"handle_message: endpoint not exists")
                }
            } else {
                error!("handle_message: EndPoint Request Message without call_id")
            }
        });
    }};
}

macro_rules! handle_endpoint_push {
    ($remote_device_id:expr, $req:tt, $handler:tt) => {{
        TOKIO_RUNTIME.spawn(async move {
            if let Err(err) = $handler($remote_device_id.clone(), $req).await {
                error!(err=?err,remote_device_id=?$remote_device_id,"handle_message: handle push message failed")
            }
        });
    }};
}

pub struct EndPoint {
    remote_device_id: String,
    atomic_call_id: AtomicU16,
    call_reply_tx_map: DashMap<u16, Sender<EndPointMessage>>,
    packet_tx: Sender<EndPointMessagePacket>,
    video_decoder_tx: OnceCell<crossbeam::channel::Sender<Vec<u8>>>,
}

impl EndPoint {
    pub async fn connect<A>(
        addr: A,
        is_active_side: bool,
        local_device_id: String,
        remote_device_id: String,
        opening_key: OpeningKey<NonceValue>,
        sealing_key: SealingKey<NonceValue>,
    ) -> Result<Self, MirrorXError>
    where
        A: ToSocketAddrs,
    {
        let mut stream = timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|_| MirrorXError::Timeout)?
            .map_err(|err| MirrorXError::IO(err))?;

        stream
            .set_nodelay(true)
            .map_err(|err| MirrorXError::IO(err))?;

        // handshake for endpoint

        let (active_device_id, passive_device_id) = if is_active_side {
            (
                format!("{:0>10}", local_device_id),
                format!("{:0>10}", remote_device_id),
            )
        } else {
            (
                format!("{:0>10}", remote_device_id),
                format!("{:0>10}", local_device_id),
            )
        };

        let active_device_id_buf = active_device_id.as_bytes();
        if active_device_id_buf.len() != 10 {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "active device id bytes length is not 10"
            )));
        }

        let passive_device_id_buf = passive_device_id.as_bytes();
        if passive_device_id_buf.len() != 10 {
            return Err(MirrorXError::Other(anyhow::anyhow!(
                "passive device id bytes length is not 10"
            )));
        }

        stream
            .write(active_device_id_buf)
            .await
            .map_err(|err| MirrorXError::IO(err))?;
        stream
            .write(passive_device_id_buf)
            .await
            .map_err(|err| MirrorXError::IO(err))?;

        let mut handshake_response_buf = [0u8; 1];
        timeout(
            Duration::from_secs(60),
            stream.read_exact(&mut handshake_response_buf),
        )
        .await
        .map_err(|_| MirrorXError::Timeout)?
        .map_err(|err| MirrorXError::IO(err))?;

        if handshake_response_buf[0] != 1 {
            return Err(MirrorXError::EndPointError(String::from(
                "handshake failed",
            )));
        }

        let framed_stream = LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(16 * 1024 * 1024)
            .new_framed(stream);

        let (sink, stream) = framed_stream.split();

        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(128);

        serve_reader(remote_device_id.clone(), stream, opening_key);
        serve_writer(remote_device_id.clone(), packet_rx, sink, sealing_key);

        Ok(Self {
            remote_device_id,
            atomic_call_id: AtomicU16::new(0),
            call_reply_tx_map: DashMap::new(),
            packet_tx,
            video_decoder_tx: OnceCell::new(),
        })
    }

    async fn call(
        &self,
        message: EndPointMessage,
        duration: Duration,
    ) -> Result<EndPointMessage, MirrorXError> {
        let call_id = self.atomic_call_id.fetch_add(1, Ordering::SeqCst);

        let packet = EndPointMessagePacket {
            typ: EndPointMessagePacketType::Request,
            call_id: Some(call_id),
            message,
        };

        let mut rx = self.register_call(call_id);
        defer! {
            self.remove_call(call_id);
        }

        timeout(duration, async move {
            if let Err(err) = self.send(packet).await {
                return Err(err);
            }

            rx.recv().await.ok_or(MirrorXError::Timeout)
        })
        .await
        .map_err(|_| MirrorXError::Timeout)?
    }

    async fn reply(&self, call_id: u16, message: EndPointMessage) -> Result<(), MirrorXError> {
        let packet = EndPointMessagePacket {
            typ: EndPointMessagePacketType::Response,
            call_id: Some(call_id),
            message,
        };

        self.send(packet).await
    }

    async fn send(&self, packet: EndPointMessagePacket) -> Result<(), MirrorXError> {
        self.packet_tx
            .try_send(packet)
            .map_err(|err| MirrorXError::Other(anyhow!(err)))
    }

    fn set_call_reply(&self, call_id: u16, message: EndPointMessage) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(message) {
                error!(err = %err,remote_device_id=?self.remote_device_id,"set_call_reply: set reply failed")
            }
        });
    }

    fn register_call(&self, call_id: u16) -> Receiver<EndPointMessage> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        self.call_reply_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u16) -> Option<Sender<EndPointMessage>> {
        self.call_reply_tx_map.remove(&call_id).map(|entry| entry.1)
    }

    pub fn begin_screen_capture(&self) -> Result<(), MirrorXError> {
        let encoder_name = if cfg!(target_os = "macos") {
            "h264_videotoolbox"
        } else if cfg!(target_os = "windows") {
            "libx264"
        } else {
            panic!("unsupported platform")
        };

        let mut video_encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

        video_encoder.set_opt("profile", "high", 0)?;
        video_encoder.set_opt("level", "5.2", 0)?;

        if encoder_name == "libx264" {
            video_encoder.set_opt("preset", "ultrafast", 0)?;
            video_encoder.set_opt("tune", "zerolatency", 0)?;
            video_encoder.set_opt("sc_threshold", "499", 0)?;
        } else {
            video_encoder.set_opt("realtime", "1", 0)?;
            video_encoder.set_opt("allow_sw", "0", 0)?;
        }

        let av_packet_rx = video_encoder.open()?;

        let (mut desktop_duplicator, capture_frame_rx) = DesktopDuplicator::new(60)?;

        std::thread::spawn(move || {
            // make sure the media_transmission after start_media_transmission send
            std::thread::sleep(Duration::from_secs(1));

            if let Err(err) = desktop_duplicator.start() {
                error!(?err, "desktop_duplicator: start capture failed");
                return;
            }

            info!("desktop_duplicator: start capture");

            loop {
                let capture_frame = match capture_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(err) => {
                        error!(?err, "capture_frame_rx: closed");
                        break;
                    }
                };

                // encode will block current thread until capture_frame released (after FFMpeg API 'avcodec_send_frame' finished)
                if let Err(err) = video_encoder.encode(capture_frame) {
                    error!(?err, "video_encoder: encode failed");
                    break;
                }
            }

            desktop_duplicator.stop();
            info!("desktop_duplicator: capture stopped");
        });

        let packet_tx = self.packet_tx.clone();

        std::thread::spawn(move || loop {
            match av_packet_rx.recv() {
                Ok(av_packet) => {
                    let packet = EndPointMessagePacket {
                        typ: EndPointMessagePacketType::Push,
                        call_id: None,
                        message: EndPointMessage::MediaFrame(MediaFrame {
                            data: av_packet.data,
                            timestamp: 0,
                        }),
                    };

                    if let Err(err) = packet_tx.try_send(packet) {
                        match err {
                            tokio::sync::mpsc::error::TrySendError::Full(_) => {
                                warn!("packet_tx: full")
                            }
                            tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                                error!("packet_tx: closed");
                                break;
                            }
                        }
                    }
                }
                Err(_) => {
                    error!("av_packet_rx: closed");
                    break;
                }
            };
        });

        Ok(())
    }

    pub fn start_desktop_render_thread(
        &self,
        texture_id: i64,
        video_texture_ptr: i64,
        update_frame_callback_ptr: i64,
    ) -> anyhow::Result<()> {
        unsafe {
            let update_callback_fn = create_callback_fn(update_frame_callback_ptr);

            let decoder_name = if cfg!(target_os = "macos") {
                "h264"
            } else if cfg!(target_os = "windows") {
                "h264_qsv"
            } else {
                panic!("unsupport platform decode");
            };

            let mut decoder = crate::media::video_decoder::VideoDecoder::new(decoder_name)?;
            // if decoder_name == "h264_qsv" {
            //     decoder.set_opt("preset", "veryfast", 0)?;
            // }

            let decode_frame_rx = decoder.open()?;

            let (decode_packet_tx, decode_packet_rx) = crossbeam::channel::bounded::<Vec<u8>>(120);

            std::thread::spawn(move || loop {
                match decode_packet_rx.recv() {
                    Ok(data) => {
                        if let Err(err) = decoder.decode(data, 0, 0) {
                            error!(?err, "video_decoder: decode failed");
                            break;
                        }
                    }
                    Err(err) => {
                        error!(?err, "decode_packet_rx: closed");
                        break;
                    }
                }
            });

            std::thread::spawn(move || loop {
                match decode_frame_rx.recv() {
                    Ok(native_frame) => {
                        #[cfg(target_os = "macos")]
                        update_callback_fn(
                            texture_id,
                            video_texture_ptr as *mut c_void,
                            native_frame.0,
                        );

                        #[cfg(target_os = "windows")]
                        update_callback_fn(
                            video_texture_ptr as *mut c_void,
                            native_frame.0.as_ptr(),
                            1920,
                            1080,
                        );
                    }
                    Err(err) => {
                        error!(?err, "decode_frame_rx: closed");
                        break;
                    }
                }
            });

            let _ = self.video_decoder_tx.set(decode_packet_tx);

            Ok(())
        }
    }

    pub fn transfer_desktop_video_frame(&self, frame: Vec<u8>) {
        if frame.len() == 0 {
            error!("transfer_desktop_video_frame: frame buffer is zero");
            return;
        }

        if let Some(decoder) = self.video_decoder_tx.get() {
            if let Err(err) = decoder.try_send(frame) {
                match err {
                    crossbeam::channel::TrySendError::Full(_) => warn!("video_decoder_rx: full"),
                    crossbeam::channel::TrySendError::Disconnected(_) => {
                        error!("video_decoder_tx: closed")
                    }
                }
            }
        }
    }

    make_endpoint_call!(
        start_media_transmission,
        StartMediaTransmissionRequest,
        EndPointMessage::StartMediaTransmissionRequest,
        StartMediaTransmissionResponse,
        EndPointMessage::StartMediaTransmissionResponse
    );
}

fn serve_reader(
    remote_device_id: String,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let mut packet_bytes = match stream.next().await {
                Some(res) => match res {
                    Ok(packet_bytes) => packet_bytes,
                    Err(err) => {
                        error!(?remote_device_id, ?err, "reader: read packet failed");
                        break;
                    }
                },
                None => {
                    info!(?remote_device_id, "reader: remote closed");
                    break;
                }
            };

            let opened_packet_bytes =
                match opening_key.open_in_place(ring::aead::Aad::empty(), &mut packet_bytes) {
                    Ok(v) => v,
                    Err(err) => {
                        error!(?remote_device_id, ?err, "reader: decrypt packet failed");
                        break;
                    }
                };

            let packet = match BINCODE_SERIALIZER
                .deserialize::<EndPointMessagePacket>(&opened_packet_bytes)
            {
                Ok(packet) => packet,
                Err(err) => {
                    error!(?remote_device_id, ?err, "reader: deserialize packet failed");
                    break;
                }
            };

            let remote_device_id = remote_device_id.clone();

            TOKIO_RUNTIME.spawn(async move {
                handle_message(remote_device_id, packet).await;
            });
        }

        ENDPOINTS.remove(&remote_device_id);
        info!(?remote_device_id, "reader: exit");
    });
}

fn serve_writer(
    remote_device_id: String,
    mut packet_rx: Receiver<EndPointMessagePacket>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            let packet = match packet_rx.recv().await {
                Some(buffer) => buffer,
                None => {
                    info!(?remote_device_id, "writer: EndPointMessagePacket tx closed");
                    break;
                }
            };

            let mut packet_buffer = match BINCODE_SERIALIZER.serialize(&packet) {
                Ok(buffer) => buffer,
                Err(err) => {
                    error!(?remote_device_id, ?err, "writer: packet serialize failed");
                    break;
                }
            };

            if let Err(err) =
                sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut packet_buffer)
            {
                error!(?remote_device_id, ?err, "writer: crypt buffer failed");
                break;
            }

            if let Err(_) = sink.send(Bytes::from(packet_buffer)).await {
                error!(?remote_device_id, "writer: send failed");
                break;
            }
        }

        ENDPOINTS.remove(&remote_device_id);
        info!(?remote_device_id, "writer: exit");
    });
}

async fn handle_message(remote_device_id: String, packet: EndPointMessagePacket) {
    match packet.typ {
        EndPointMessagePacketType::Request => match packet.message {
            EndPointMessage::StartMediaTransmissionRequest(req) => {
                handle_endpoint_call!(
                    remote_device_id,
                    packet.call_id,
                    req,
                    EndPointMessage::StartMediaTransmissionResponse,
                    handle_start_media_transmission_request
                )
            }
            _ => error!("handle_message: received unexpected EndPoint Request Message"),
        },
        EndPointMessagePacketType::Response => {
            if let Some(call_id) = packet.call_id {
                if let Some(endpoint) = ENDPOINTS.get(&remote_device_id) {
                    endpoint.set_call_reply(call_id, packet.message);
                }
            } else {
                error!("handle_message: EndPoint Response Message without call_id")
            }
        }
        EndPointMessagePacketType::Push => match packet.message {
            EndPointMessage::MediaFrame(req) => {
                handle_endpoint_push!(remote_device_id, req, handle_media_transmission)
            }
            _ => error!("handle_message: received unexpected EndPoint Push Message"),
        },
    }
}
