mod crypto;
mod db;
mod generator;
mod commands;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub key: Mutex<Option<[u8; 32]>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_data = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data).expect("failed to create app data dir");
            let db_path = app_data.join("password_random.db");
            let conn = Connection::open(&db_path).expect("failed to open database");
            db::init_tables(&conn).expect("failed to initialize tables");
            app.manage(AppState {
                db: Mutex::new(conn),
                key: Mutex::new(None),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::is_initialized,
            commands::init_master_key,
            commands::unlock,
            commands::is_locked,
            commands::lock,
            commands::change_master_password,
            commands::generate_password,
            commands::generate_pin,
            commands::generate_passphrase,
            commands::generate_from_template,
            commands::validate_template,
            commands::save_password,
            commands::get_passwords,
            commands::delete_password,
            commands::get_categories,
            commands::create_category,
            commands::delete_category,
            commands::get_templates,
            commands::save_template,
            commands::delete_template,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
