use crate::{
    component::display::Monitor,
    error::MirrorXError,
    ffi::os::*,
    utility::wide_char::{FromWide, ToWide},
};
use image::ColorType;
use libc::c_void;
use scopeguard::defer;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    io::Cursor,
    mem::zeroed,
    ops::{Deref, DerefMut},
    os::windows::prelude::OsStringExt,
};
use tracing::{error, info};
use windows::{
    core::{Abi, Interface, PCWSTR},
    Win32::{
        Devices::Display::{
            DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes,
            GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR,
            QueryDisplayConfig, DISPLAYCONFIG_ADAPTER_NAME,
            DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO,
            DISPLAYCONFIG_MODE_INFO_TYPE, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET,
            DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
            DISPLAYCONFIG_TARGET_DEVICE_NAME,
        },
        Foundation::{BOOL, ERROR_SUCCESS, HWND, LPARAM, LUID, RECT},
        Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_REFERENCE,
                D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0,
                D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11Texture2D, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION,
            },
            Dxgi::{IDXGIAdapter, IDXGIDevice, IDXGIOutput1},
            Gdi::{
                BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateDCW, CreateDIBSection,
                DeleteObject, EnumDisplayDevicesW, EnumDisplayMonitors, EnumDisplaySettingsW,
                GetBitmapBits, GetDC, GetDIBits, GetMonitorInfoW, GetObjectA, GetObjectW,
                ReleaseDC, SelectObject, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, CAPTUREBLT,
                DEVMODEW, DIB_RGB_COLORS, DISPLAY_DEVICEW, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP,
                DISPLAY_DEVICE_PRIMARY_DEVICE, ENUM_CURRENT_SETTINGS, HDC, HGDIOBJ, HMONITOR,
                MONITORENUMPROC, MONITORINFO, MONITORINFOEXA, MONITORINFOEXW, QDC_ALL_PATHS,
                QDC_ONLY_ACTIVE_PATHS, ROP_CODE, SRCCOPY,
            },
        },
        UI::WindowsAndMessaging::{
            GetSystemMetrics, EDD_GET_DEVICE_INTERFACE_NAME, MONITORINFOF_PRIMARY, SM_CMONITORS,
        },
    },
};

pub fn get_active_displays() -> Result<Vec<Monitor>, MirrorXError> {
    unsafe {
        let all_monitors = enum_all_monitors_path_and_name()?;
        let dxgi_output_monitors = enum_dxgi_outputs(all_monitors)?;

        Ok(dxgi_output_monitors)
    }
}

unsafe fn enum_dxgi_outputs(
    all_monitors: HashMap<String, String>,
) -> Result<Vec<Monitor>, MirrorXError> {
    let driver_types = [
        D3D_DRIVER_TYPE_HARDWARE,
        D3D_DRIVER_TYPE_WARP,
        D3D_DRIVER_TYPE_REFERENCE,
        // D3D_DRIVER_TYPE_UNKNOWN,
    ];

    let feature_levels = [
        D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL_11_0,
        D3D_FEATURE_LEVEL_10_1,
        D3D_FEATURE_LEVEL_10_0,
    ];

    let mut device = None;
    let mut feature_level = std::mem::zeroed();

    for driver_type in driver_types {
        if D3D11CreateDevice(
            None,
            driver_type,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG,
            &feature_levels,
            D3D11_SDK_VERSION,
            &mut device,
            &mut feature_level,
            std::ptr::null_mut(),
        )
        .is_ok()
        {
            info!(driver_type=?get_driver_type_name(driver_type), feature_level=?get_d3d_feature_level_name(feature_level),"create d3d device");
            break;
        };
    }

    let device = if let Some(device) = device {
        device
    } else {
        return Err(MirrorXError::D3D {
            api_name: Some("D3D11CreateDevice"),
            description: Some(String::from(
                "create d3d device failed with all kind of driver types",
            )),
            error_code: None,
        });
    };

    let dxgi_device: IDXGIDevice = device.cast().map_err(|err| MirrorXError::D3D {
        api_name: Some("ID3D11Device::QueryInterface"),
        description: Some(format!(
            "ID3D11Device query interface as IDXGIDevice failed ({})",
            err
        )),
        error_code: None,
    })?;

    let dxgi_adapter: IDXGIAdapter = dxgi_device.GetParent().map_err(|err| MirrorXError::D3D {
        api_name: Some("IDXGIDevice::GetParent"),
        description: Some(format!("IDXGIDevice get parent failed ({})", err)),
        error_code: None,
    })?;

    let adapter_desc = dxgi_adapter.GetDesc().map_err(|err| MirrorXError::D3D {
        api_name: Some("IDXGIAdapter::GetDesc"),
        description: Some(format!("IDXGIAdapter get desc failed ({})", err)),
        error_code: None,
    })?;

    info!(
        adapter_name = ?OsString::from_wide_null(&adapter_desc.Description),
        "enum dxgi outputs adapter"
    );

    let mut displays = Vec::new();
    let mut output_index = 0u32;

    loop {
        let dxgi_output = if let Ok(dxgi_output) = dxgi_adapter.EnumOutputs(output_index) {
            output_index += 1;
            dxgi_output
        } else {
            break;
        };

        let output_desc = dxgi_output.GetDesc().map_err(|err| MirrorXError::D3D {
            api_name: Some("IDXGIOutput::GetDesc"),
            description: Some(format!("IDXGIOutput get desc failed ({})", err)),
            error_code: None,
        })?;

        if !output_desc.AttachedToDesktop.as_bool() {
            continue;
        }

        let mut monitor_info: MONITORINFO = std::mem::zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

        let mut monitor_is_primary = false;
        let mut screen_width = 0;
        let mut screen_height = 0;
        if GetMonitorInfoW(output_desc.Monitor, &mut monitor_info as *mut _).as_bool() {
            monitor_is_primary = (monitor_info.dwFlags & MONITORINFOF_PRIMARY) != 0;
            screen_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
            screen_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
        }

        let mut dev_index = 0u32;
        loop {
            let origin_device_name = OsString::from_wide_null(&output_desc.DeviceName);

            let mut display_device: DISPLAY_DEVICEW = std::mem::zeroed();
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            let success = EnumDisplayDevicesW(
                &*origin_device_name,
                dev_index,
                &mut display_device as *mut _,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
            .as_bool();

            dev_index += 1;

            if !success {
                break;
            }

            let mut display_mode: DEVMODEW = std::mem::zeroed();
            display_mode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            let mut refresh_rate = 0;
            let mut physical_width = 0;
            let mut physical_height = 0;

            if EnumDisplaySettingsW(
                &*origin_device_name,
                ENUM_CURRENT_SETTINGS,
                &mut display_mode as *mut _,
            )
            .as_bool()
            {
                refresh_rate = display_mode.dmDisplayFrequency;
                physical_width = display_mode.dmPelsWidth;
                physical_height = display_mode.dmPelsHeight;
            }

            if (display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP) != 0 {
                let screent_shot_buffer = take_screen_shot(
                    origin_device_name,
                    monitor_info.rcMonitor.left,
                    monitor_info.rcMonitor.top,
                    physical_width,
                    physical_height,
                )?;

                let device_id = OsString::from_wide_null(&display_device.DeviceID)
                    .into_string()
                    .map_err(|_| {
                        MirrorXError::Other(anyhow::anyhow!("convert OsString to String failed"))
                    })?;

                let name = all_monitors
                    .get(&device_id)
                    .map_or(String::default(), |name| name.clone());

                displays.push(Monitor {
                    id: device_id,
                    name,
                    refresh_rate: refresh_rate.to_string(),
                    width: screen_width as u16,
                    height: screen_height as u16,
                    main: monitor_is_primary,
                    screen_shot: screent_shot_buffer,
                });
            }
        }
    }

    Ok(displays)
}

unsafe fn enum_all_monitors_path_and_name() -> Result<HashMap<String, String>, MirrorXError> {
    let mut path_count: u32 = 0;
    let mut mode_count: u32 = 0;
    let mut err_code = GetDisplayConfigBufferSizes(
        QDC_ALL_PATHS,
        &mut path_count as *mut _,
        &mut mode_count as *mut _,
    );

    if err_code != (ERROR_SUCCESS.0 as i32) {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "GetDisplayConfigBufferSizes error ({})",
            err_code
        )));
    }

    let mut display_paths = Vec::<DISPLAYCONFIG_PATH_INFO>::with_capacity(path_count as usize);
    let mut display_modes = Vec::<DISPLAYCONFIG_MODE_INFO>::with_capacity(mode_count as usize);

    err_code = QueryDisplayConfig(
        QDC_ALL_PATHS,
        &mut path_count as *mut _,
        display_paths.as_mut_ptr() as *mut _,
        &mut mode_count as *mut _,
        display_modes.as_mut_ptr() as *mut _,
        std::ptr::null_mut(),
    );

    if err_code != (ERROR_SUCCESS.0 as i32) {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "QueryDisplayConfig error ({})",
            err_code
        )));
    }

    display_paths.set_len(path_count as usize);
    display_modes.set_len(mode_count as usize);

    let mut all_monitors_path_and_name = HashMap::new();

    for mode_info in display_modes {
        if (mode_info.infoType == DISPLAYCONFIG_MODE_INFO_TYPE_TARGET) {
            let mut device_name: DISPLAYCONFIG_TARGET_DEVICE_NAME = std::mem::zeroed();
            device_name.header = DISPLAYCONFIG_DEVICE_INFO_HEADER {
                size: std::mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                adapterId: mode_info.adapterId,
                id: mode_info.id,
                r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
            };

            let device_name_header_ptr = std::mem::transmute::<
                *mut DISPLAYCONFIG_TARGET_DEVICE_NAME,
                *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
            >(&mut device_name as *mut _);

            let err_code = DisplayConfigGetDeviceInfo(device_name_header_ptr);
            if err_code != (ERROR_SUCCESS.0 as i32) {
                return Err(MirrorXError::Other(anyhow::anyhow!(
                    "DisplayConfigGetDeviceInfo error ({})",
                    err_code
                )));
            }

            let device_path = OsString::from_wide_null(device_name.monitorDevicePath.as_ref())
                .into_string()
                .map_err(|_| {
                    MirrorXError::Other(anyhow::anyhow!(
                        "convert monitorDevicePath to string failed"
                    ))
                })?;

            let device_friendly_name =
                OsString::from_wide_null(device_name.monitorFriendlyDeviceName.as_ref())
                    .into_string()
                    .map_err(|_| {
                        MirrorXError::Other(anyhow::anyhow!(
                            "convert monitorFriendlyDeviceName to string failed"
                        ))
                    })?;

            all_monitors_path_and_name.insert(device_path, device_friendly_name);
        }
    }

    Ok(all_monitors_path_and_name)
}

unsafe fn take_screen_shot(
    device_name: OsString,
    monitor_coord_x: i32,
    monitor_coord_y: i32,
    physical_width: u32,
    physical_height: u32,
) -> Result<Vec<u8>, MirrorXError> {
    let src_dc = CreateDCW("", device_name, "", std::ptr::null());
    if src_dc.is_invalid() {
        return Err(MirrorXError::Other(anyhow::anyhow!("CreateDCW failed")));
    }

    defer! {
        ReleaseDC(None, src_dc);
    }

    let dst_dc = CreateCompatibleDC(src_dc);
    if dst_dc.is_invalid() {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "CreateCompatibleBitmap failed"
        )));
    }

    defer! {
        DeleteObject(HGDIOBJ(dst_dc.0));
    }

    let src_bitmap = CreateCompatibleBitmap(src_dc, physical_width as i32, physical_height as i32);

    defer! {
        DeleteObject(src_bitmap);
    }

    SelectObject(dst_dc, src_bitmap);

    if !BitBlt(
        dst_dc,
        0,
        0,
        physical_width as i32,
        physical_height as i32,
        src_dc,
        monitor_coord_x,
        monitor_coord_y,
        ROP_CODE(SRCCOPY.0 | CAPTUREBLT.0),
    )
    .as_bool()
    {
        return Err(MirrorXError::Other(anyhow::anyhow!("BitBlt failed")));
    }

    let mut bitmap: BITMAP = std::mem::zeroed();

    if GetObjectW(
        src_bitmap,
        std::mem::size_of::<BITMAP>() as i32,
        &mut bitmap as *mut _ as *mut c_void,
    ) == 0
    {
        return Err(MirrorXError::Other(anyhow::anyhow!("GetObjectW failed")));
    }

    let is_32_bit = bitmap.bmBitsPixel == 32;
    let mut bmp_bytes = Vec::<u8>::with_capacity((bitmap.bmWidthBytes * bitmap.bmHeight) as usize);

    if GetBitmapBits(
        src_bitmap,
        bitmap.bmWidthBytes * bitmap.bmHeight,
        bmp_bytes.as_mut_ptr() as *mut c_void,
    ) == 0
    {
        return Err(MirrorXError::Other(anyhow::anyhow!("GetBitmapBits failed")));
    }

    bmp_bytes.set_len((bitmap.bmWidthBytes * bitmap.bmHeight) as usize);

    // swap blue(at index 0) and red(at index 2) color byte to convert BGRA(or BGR) order to RGBA(or RGB) order
    // every chunk size is 4 for BGRA or 3 for BGR
    let chunk_size = if is_32_bit { 4 } else { 3 };

    for chunk in bmp_bytes.chunks_mut(chunk_size).into_iter() {
        chunk[0] = chunk[0] ^ chunk[2];
        chunk[2] = chunk[0] ^ chunk[2];
        chunk[0] = chunk[0] ^ chunk[2];
    }

    let mut png_bytes: Vec<u8> = Vec::with_capacity(bmp_bytes.len());

    if let Err(err) = image::write_buffer_with_format(
        &mut Cursor::new(&mut png_bytes),
        &bmp_bytes,
        bitmap.bmWidth as u32,
        bitmap.bmHeight as u32,
        if is_32_bit {
            ColorType::Rgba8
        } else {
            ColorType::Rgb8
        },
        image::ImageOutputFormat::Png,
    ) {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "write image failed ({})",
            err
        )));
    }

    Ok(png_bytes)
}

#[allow(unused)]
fn get_driver_type_name(driver_type: D3D_DRIVER_TYPE) -> &'static str {
    match driver_type {
        D3D_DRIVER_TYPE_UNKNOWN => "D3D_DRIVER_TYPE_UNKNOWN",
        D3D_DRIVER_TYPE_HARDWARE => "D3D_DRIVER_TYPE_HARDWARE",
        D3D_DRIVER_TYPE_REFERENCE => "D3D_DRIVER_TYPE_REFERENCE",
        D3D_DRIVER_TYPE_NULL => "D3D_DRIVER_TYPE_NULL",
        D3D_DRIVER_TYPE_SOFTWARE => "D3D_DRIVER_TYPE_SOFTWARE",
        D3D_DRIVER_TYPE_WARP => "D3D_DRIVER_TYPE_WARP",
        _ => "Unknown",
    }
}

#[allow(unused)]
fn get_d3d_feature_level_name(feature_level: D3D_FEATURE_LEVEL) -> &'static str {
    match feature_level {
        D3D_FEATURE_LEVEL_12_2 => "D3D_FEATURE_LEVEL_12_2",
        D3D_FEATURE_LEVEL_12_1 => "D3D_FEATURE_LEVEL_12_1",
        D3D_FEATURE_LEVEL_12_0 => "D3D_FEATURE_LEVEL_12_0",
        D3D_FEATURE_LEVEL_11_1 => "D3D_FEATURE_LEVEL_11_1",
        D3D_FEATURE_LEVEL_11_0 => "D3D_FEATURE_LEVEL_11_0",
        D3D_FEATURE_LEVEL_10_1 => "D3D_FEATURE_LEVEL_10_1",
        D3D_FEATURE_LEVEL_10_0 => "D3D_FEATURE_LEVEL_10_0",
        D3D_FEATURE_LEVEL_9_3 => "D3D_FEATURE_LEVEL_9_3",
        D3D_FEATURE_LEVEL_9_2 => "D3D_FEATURE_LEVEL_9_2",
        D3D_FEATURE_LEVEL_9_1 => "D3D_FEATURE_LEVEL_9_1",
        D3D_FEATURE_LEVEL_1_0_CORE => "D3D_FEATURE_LEVEL_1_0_CORE",
        _ => "Unknown",
    }
}
