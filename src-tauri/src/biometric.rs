use crate::AppState;
use crate::crypto;
use crate::db;

fn derive_wrapping_key(salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    argon2::Argon2::default()
        .hash_password_into(b"PasswordRandomBio2026", salt, &mut key)
        .expect("argon2 wrapping key failed");
    key
}

pub fn enable(state: &AppState) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    let enc_key = key_state.as_ref().ok_or("应用未解锁")?;

    let salt = crypto::generate_salt();
    let wrapping_key = derive_wrapping_key(&salt);
    let (ciphertext, nonce) = crypto::encrypt(&wrapping_key, enc_key)?;
    db::save_biometric_config(&db, &ciphertext, &nonce, &salt)?;
    Ok(())
}

pub fn disable(state: &AppState) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::delete_biometric_config(&db)?;
    Ok(())
}

pub fn is_enabled(state: &AppState) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    db::has_biometric_config(&db)
}

pub fn unlock(state: &AppState) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("{e}"))?;
    let config = db::get_biometric_config(&db)?;
    let config = config.ok_or("未启用指纹解锁")?;

    let wrapping_key = derive_wrapping_key(&config.salt);
    let plain = crypto::decrypt(&wrapping_key, &config.ciphertext, &config.nonce)?;
    let enc_key: [u8; 32] = plain
        .try_into()
        .map_err(|_| "stored key corrupted".to_string())?;

    drop(db);
    let mut key_state = state.key.lock().map_err(|e| format!("{e}"))?;
    *key_state = Some(enc_key);
    Ok(())
}
