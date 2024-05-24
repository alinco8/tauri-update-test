// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle().clone();
            let handle2 = app.handle().clone();
            handle.get_webview_window("main").unwrap().open_devtools();

            tauri::async_runtime::spawn(async move {
                println!("checking...");
                if let Some(update) = handle.updater().unwrap().check().await.unwrap() {
                    println!("has_update...");

                    if update
                        .download_and_install(
                            |_us, _option| {
                                println!("chunk received");
                            },
                            || {
                                println!("download finished");
                            },
                        )
                        .await
                        .is_ok()
                    {
                        println!("install ended...");
                        tauri::process::restart(&handle2.env());
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
