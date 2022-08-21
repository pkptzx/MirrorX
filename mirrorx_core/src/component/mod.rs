#![allow(non_snake_case)]
mod macros;

pub mod audio_decoder;
pub mod audio_encoder;
pub mod capture_frame;
pub mod desktop;
pub mod input;
pub mod monitor;
pub mod video_decoder;
pub mod video_encoder;

pub const NALU_HEADER_LENGTH: usize = 4;

#[cfg(target_os = "windows")]
pub mod media_foundation;
