use std::sync::RwLock;

use app_state::AppState;
use data::Data;
use serde_json::json;
use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_store::StoreExt;
use totp::TotpConfig;

pub mod app_state;
pub mod data;
pub mod totp;

#[tauri::command]
async fn add_data(
    app: tauri::AppHandle,
    title: &str,
    sub_title: Option<String>,
    secret_code: &str,
) -> Result<String, String> {
    let result = format!("{} {:?} {}", title, sub_title, secret_code);
    let data = Data::new(title, sub_title, secret_code);
    data.store(app.clone()).map_err(|e| e.to_string())?;
    let config = TotpConfig::new(&data.secret_code);
    let state = app.state::<RwLock<AppState>>().clone();
    let mut state = state.write().unwrap();
    state.config = Some(config);
    Ok(result)
}

#[tauri::command]
async fn get_code(app: tauri::AppHandle) -> Result<Option<(String, u64)>, String> {
    let state = app.state::<RwLock<AppState>>().clone();
    let state = state.read().unwrap();

    if let Some(config) = &state.config {
        let code = config.generate_code().map_err(|e| format!("{:?}", e))?;
        let time = config.get_remaining_seconds();

        return Ok(Some((code, time)));
    }

    Ok(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = show_window(app);
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .manage(RwLock::new(app_state))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![add_data, get_code])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // 开机自动启动
            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();

            // 检查 enable 状态
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );

            // 托盘
            let _tray = TrayIconBuilder::new()
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => { /* Do nothing */ }
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            // 持久化
            let store = app.store("app_data.json")?;

            let secret_code = if let Some(data) = store.get("MyKey") {
                let data: Data = serde_json::from_value(data)?;
                data.secret_code
            } else {
                let data = Data::new("default", None, "SECRETKEYBASE32ENCODEDTEST");
                store.set("MyKey", json!(data));
                data.secret_code
            };

            let config = TotpConfig::new(&secret_code);
            let state = app.state::<RwLock<AppState>>().clone();
            let mut state = state.write().unwrap();
            state.config = Some(config);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent {
                label,
                event: win_event,
                ..
            } => match win_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let win = app.get_webview_window(label.as_str()).unwrap();
                    let _ = win.hide();
                    api.prevent_close();
                }
                _ => {}
            },
            _ => {}
        })
}

fn show_window(app: &AppHandle) {
    let windows = app.webview_windows();

    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
}
