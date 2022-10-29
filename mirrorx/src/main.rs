#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod api;
mod platform;
mod utility;

fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(api::UIState::default())
        .setup(|app| {
            if let Some(win) = app.get_window("main") {
                #[cfg(target_os = "macos")]
                {
                    use platform::window_ext::WindowExt;
                    win.expand_title_bar();
                }
            }

            Ok(())
        })
        // invoke_handler should be called only once!
        .invoke_handler(tauri::generate_handler![
            api::init_config,
            api::get_config_primary_domain,
            api::get_config_device_id,
            api::get_config_device_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}