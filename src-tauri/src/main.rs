#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use database::db::init_db;
use diesel::SqliteConnection;
use handler::*;
use system_tray::{create_system_tray, handler_system_tray_event};
use tauri::Manager;

mod database;
mod error;
mod handler;
mod system_tray;
mod utils;

pub struct UserManagementWrapper(pub Mutex<UserManagement>);

// 用户配置
pub struct UserManagement {
    pub conn: SqliteConnection,
}

fn main() {
    let conn = init_db();
    let new_system_tray = create_system_tray();

    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .manage(UserManagementWrapper(Mutex::new(UserManagement { conn })))
        .invoke_handler(tauri::generate_handler![
            get_project_item_list,
            add_project_item,
            update_project_item,
            update_project_item_property,
            delete_project_item,
            get_scheme_by_id,
            get_start_scheme_list,
            add_start_scheme,
            update_start_scheme,
            delete_start_scheme,
            open_program,
            execute_command
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .system_tray(new_system_tray)
        .on_system_tray_event(handler_system_tray_event)
        .run(ctx)
        .expect("error while running tauri application");
}
