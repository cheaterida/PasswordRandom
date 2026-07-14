mod crypto;
mod db;
mod generator;
mod commands;
mod biometric;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub key: Mutex<Option<[u8; 32]>>,
    pub db_path: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init());

    #[cfg(mobile)]
    {
        builder = builder.plugin(tauri_plugin_biometric::init());
    }

    builder
        .setup(|app| {
            let app_data = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data).expect("failed to create app data dir");
            let db_path = app_data.join("password_random.db");
            let db_path_str = db_path.to_string_lossy().to_string();
            let conn = Connection::open(&db_path).expect("failed to open database");
            db::init_tables(&conn).expect("failed to initialize tables");
            app.manage(AppState {
                db: Mutex::new(conn),
                key: Mutex::new(None),
                db_path: db_path_str,
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
            commands::update_template,
            commands::biometric_is_available,
            commands::biometric_enable,
            commands::biometric_disable,
            commands::biometric_is_enabled,
            commands::biometric_unlock,
            commands::get_db_path,
            commands::get_preference,
            commands::set_preference,
            commands::toggle_favorite,
            commands::delete_passwords,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
