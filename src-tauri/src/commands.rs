use tauri::State;
use zeroize::Zeroize;
use crate::db::{self, Category, PasswordDisplay, Template};
use crate::generator::{self, GenConfig, PinConfig, PassphraseConfig};
use crate::crypto;
use crate::biometric;
use crate::AppState;
use base64::Engine;

#[tauri::command]
pub async fn is_initialized(state: State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::is_initialized(&db)
}

#[tauri::command]
pub async fn init_master_key(
    state: State<'_, AppState>,
    password: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let initialized = db::is_initialized(&db)?;
    if initialized {
        return Err("主密码已经设置，请先解锁后使用修改密码功能".to_string());
    }

    let salt1 = crypto::generate_salt();
    let salt2 = crypto::generate_salt();
    let key = crypto::derive_key(&password, &salt1);
    let mut verification = [0u8; 32];
    argon2::Argon2::default()
        .hash_password_into(&key, &salt2, &mut verification)
        .map_err(|e| format!("argon2 error: {e}"))?;

    db::save_master_config(&db, &salt1, &salt2, &verification)?;
    drop(db);

    let mut key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    *key_state = Some(key);
    Ok(())
}

#[tauri::command]
pub async fn unlock(state: State<'_, AppState>, password: String) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let config = db::get_master_config(&db)?;
    let config = config.ok_or("未设置主密码")?;

    let valid = crypto::verify_master_key(&password, &config.salt1, &config.salt2, &config.verification_hash);
    if valid {
        let key = crypto::derive_key(&password, &config.salt1);
        drop(db);
        let mut key_state = state.key.lock().map_err(|e| format!("{e}"))?;
        *key_state = Some(key);
    }
    Ok(valid)
}

#[tauri::command]
pub async fn is_locked(state: State<'_, AppState>) -> Result<bool, String> {
    let key = state.key.lock().map_err(|e| format!("{e}"))?;
    Ok(key.is_none())
}

#[tauri::command]
pub async fn lock(state: State<'_, AppState>) -> Result<(), String> {
    let mut key = state.key.lock().map_err(|e| format!("{e}"))?;
    key.zeroize();
    *key = None;
    Ok(())
}

#[tauri::command]
pub async fn change_master_password(
    state: State<'_, AppState>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let config = db::get_master_config(&db)?;
    let config = config.ok_or("未设置主密码")?;

    let valid = crypto::verify_master_key(
        &old_password,
        &config.salt1,
        &config.salt2,
        &config.verification_hash,
    );
    if !valid {
        return Err("旧密码错误".to_string());
    }

    let old_key = crypto::derive_key(&old_password, &config.salt1);

    // Re-encrypt all passwords with new key
    let new_salt1 = crypto::generate_salt();
    let new_salt2 = crypto::generate_salt();
    let new_key = crypto::derive_key(&new_password, &new_salt1);
    let mut new_verification = [0u8; 32];
    argon2::Argon2::default()
        .hash_password_into(&new_key, &new_salt2, &mut new_verification)
        .map_err(|e| format!("argon2 error: {e}"))?;

    // Re-encrypt all existing passwords
    re_encrypt_all(&db, &old_key, &new_key)?;

    db::delete_master_config(&db)?;
    db::save_master_config(&db, &new_salt1, &new_salt2, &new_verification)?;
    drop(db);

    let mut key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    *key_state = Some(new_key);
    Ok(())
}

fn re_encrypt_all(
    conn: &rusqlite::Connection,
    old_key: &[u8; 32],
    new_key: &[u8; 32],
) -> Result<(), String> {
    let mut stmt = conn
        .prepare("SELECT id, ciphertext, nonce FROM password_history")
        .map_err(|e| format!("{e}"))?;

    let entries: Vec<(i64, Vec<u8>, Vec<u8>)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(|e| format!("{e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("{e}"))?;

    for (id, ciphertext, nonce) in entries {
        let plain = crypto::decrypt(old_key, &ciphertext, &nonce)?;
        let (new_ciphertext, new_nonce) = crypto::encrypt(new_key, &plain)?;
        conn.execute(
            "UPDATE password_history SET ciphertext = ?1, nonce = ?2 WHERE id = ?3",
            rusqlite::params![new_ciphertext, new_nonce, id],
        )
        .map_err(|e| format!("{e}"))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn generate_password(config: GenConfig) -> Result<String, String> {
    generator::generate_random(&config)
}

#[tauri::command]
pub async fn generate_pin(config: PinConfig) -> Result<String, String> {
    generator::generate_pin(&config)
}

#[tauri::command]
pub async fn generate_passphrase(config: PassphraseConfig) -> Result<String, String> {
    Ok(generator::generate_passphrase(&config))
}

#[tauri::command]
pub async fn generate_from_template(pattern: String) -> Result<String, String> {
    generator::generate_from_template(&pattern)
}

#[tauri::command]
pub async fn validate_template(pattern: String) -> Result<bool, String> {
    match generator::validate_template(&pattern) {
        Ok(()) => Ok(true),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn save_password(
    state: State<'_, AppState>,
    label: String,
    username: String,
    password: String,
    mode: String,
    category_id: Option<i64>,
    template_id: Option<i64>,
) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    let key = key_state.as_ref().ok_or("应用未解锁")?;
    let id = db::save_password(&db, key, &label, &username, &password, &mode, category_id, template_id)?;
    Ok(id)
}

#[tauri::command]
pub async fn get_passwords(
    state: State<'_, AppState>,
    category_id: Option<i64>,
    search: Option<String>,
) -> Result<Vec<PasswordDisplay>, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    let key = key_state.as_ref().ok_or("应用未解锁")?;
    db::get_passwords(&db, key, category_id, search.as_deref())
}

#[tauri::command]
pub async fn delete_password(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::delete_password(&db, id)
}

#[tauri::command]
pub async fn get_categories(state: State<'_, AppState>) -> Result<Vec<Category>, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::get_categories(&db)
}

#[tauri::command]
pub async fn create_category(
    state: State<'_, AppState>,
    name: String,
    icon: String,
) -> Result<Category, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::create_category(&db, &name, &icon)
}

#[tauri::command]
pub async fn delete_category(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::delete_category(&db, id)
}

#[tauri::command]
pub async fn get_templates(state: State<'_, AppState>) -> Result<Vec<Template>, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::get_templates(&db)
}

#[tauri::command]
pub async fn save_template(
    state: State<'_, AppState>,
    name: String,
    pattern: String,
) -> Result<Template, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::save_template(&db, &name, &pattern)
}

#[tauri::command]
pub async fn delete_template(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::delete_template(&db, id)
}

#[tauri::command]
pub async fn update_template(state: State<'_, AppState>, id: i64, name: String, pattern: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::update_template(&db, id, &name, &pattern)
}

#[tauri::command]
pub async fn biometric_is_available() -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
pub async fn biometric_enable(state: State<'_, AppState>) -> Result<(), String> {
    biometric::enable(&state)
}

#[tauri::command]
pub async fn biometric_disable(state: State<'_, AppState>) -> Result<(), String> {
    biometric::disable(&state)
}

#[tauri::command]
pub async fn biometric_is_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    biometric::is_enabled(&state)
}

#[tauri::command]
pub async fn biometric_unlock(state: State<'_, AppState>) -> Result<(), String> {
    biometric::unlock(&state)
}

#[tauri::command]
pub async fn get_db_path(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.db_path.clone())
}

#[tauri::command]
pub async fn get_preference(state: State<'_, AppState>, key: String) -> Result<Option<String>, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::get_preference(&db, &key)
}

#[tauri::command]
pub async fn set_preference(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::set_preference(&db, &key, &value)
}

#[tauri::command]
pub async fn toggle_favorite(state: State<'_, AppState>, id: i64) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::toggle_favorite(&db, id)
}

#[tauri::command]
pub async fn delete_passwords(state: State<'_, AppState>, ids: Vec<i64>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::delete_passwords(&db, &ids)
}

#[tauri::command]
pub async fn export_data(state: State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    let key = key_state.as_ref().ok_or("应用未解锁")?;

    let passwords = db::get_passwords(&db, key, None, None)?;
    let categories = db::get_categories(&db)?;

    let entries: Vec<serde_json::Value> = passwords.iter().map(|p| {
        serde_json::json!({
            "label": p.label,
            "username": p.username,
            "password": p.password,
            "category": p.category_name,
            "mode": p.mode,
            "created_at": p.created_at,
        })
    }).collect();

    let cats: Vec<serde_json::Value> = categories.iter().map(|c| {
        serde_json::json!({
            "name": c.name,
            "icon": c.icon,
        })
    }).collect();

    let export = serde_json::json!({
        "version": 1,
        "app": "PasswordRandom",
        "exported_at": format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)),
        "categories": cats,
        "entries": entries,
    });

    let plaintext = serde_json::to_vec(&export).map_err(|e| format!("序列化失败: {e}"))?;
    let (ciphertext, nonce) = crypto::encrypt(key, &plaintext)?;
    let mut output = nonce.to_vec();
    output.extend_from_slice(&ciphertext);
    Ok(base64::engine::general_purpose::STANDARD.encode(&output))
}
