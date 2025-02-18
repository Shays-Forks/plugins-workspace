// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
#[cfg(desktop)]
mod tray;

use serde::Serialize;
use tauri::{window::WindowBuilder, App, AppHandle, RunEvent, WindowUrl};

#[derive(Clone, Serialize)]
struct Reply {
    data: String,
}

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;
pub type OnEvent = Box<dyn FnMut(&AppHandle, RunEvent)>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_app::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window::init())
        .setup(move |app| {
            #[cfg(desktop)]
            {
                tray::create_tray(app)?;
                app.handle().plugin(tauri_plugin_cli::init())?;
                app.handle()
                    .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
                app.handle()
                    .plugin(tauri_plugin_updater::Builder::new().build())?;
            }

            let mut window_builder = WindowBuilder::new(app, "main", WindowUrl::default());
            #[cfg(desktop)]
            {
                window_builder = window_builder
                    .user_agent("Tauri API")
                    .title("Tauri API Validation")
                    .inner_size(1000., 800.)
                    .min_inner_size(600., 400.)
                    .content_protected(true);
            }

            #[cfg(target_os = "windows")]
            {
                window_builder = window_builder
                    .transparent(true)
                    .shadow(true)
                    .decorations(false);
            }

            let window = window_builder.build().unwrap();

            #[cfg(debug_assertions)]
            window.open_devtools();

            #[cfg(desktop)]
            std::thread::spawn(|| {
                let server = match tiny_http::Server::http("localhost:3003") {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                };
                loop {
                    if let Ok(mut request) = server.recv() {
                        let mut body = Vec::new();
                        let _ = request.as_reader().read_to_end(&mut body);
                        let response = tiny_http::Response::new(
                            tiny_http::StatusCode(200),
                            request.headers().to_vec(),
                            std::io::Cursor::new(body),
                            request.body_length(),
                            None,
                        );
                        let _ = request.respond(response);
                    }
                }
            });

            Ok(())
        })
        .on_page_load(|window, _| {
            let window_ = window.clone();
            window.listen("js-event", move |event| {
                println!("got js-event with message '{:?}'", event.payload());
                let reply = Reply {
                    data: "something else".to_string(),
                };

                window_
                    .emit("rust-event", Some(reply))
                    .expect("failed to emit");
            });
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.menu(tauri::Menu::os_default("Tauri API Validation"));
    }

    #[allow(unused_mut)]
    let mut app = builder
        .invoke_handler(tauri::generate_handler![
            cmd::log_operation,
            cmd::perform_request,
        ])
        .build(tauri::tauri_build_context!())
        .expect("error while building tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Regular);

    app.run(move |_app_handle, _event| {
        #[cfg(desktop)]
        if let RunEvent::ExitRequested { api, .. } = &_event {
            // Keep the event loop running even if all windows are closed
            // This allow us to catch system tray events when there is no window
            api.prevent_exit();
        }
    })
}
