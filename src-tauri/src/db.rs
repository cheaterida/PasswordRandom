use rusqlite::{params, Connection};
use crate::crypto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub created_at: String,
}

pub fn toggle_favorite(conn: &Connection, id: i64) -> Result<bool, String> {
    conn.execute(
        "UPDATE password_history SET is_favorite = CASE WHEN is_favorite = 0 THEN 1 ELSE 0 END WHERE id = ?1",
        params![id],
    )
    .map_err(|e| format!("{e}"))?;
    let fav: i32 = conn
        .query_row("SELECT is_favorite FROM password_history WHERE id = ?1", params![id], |row| row.get(0))
        .unwrap_or(0);
    Ok(fav != 0)
}

pub fn delete_passwords(conn: &Connection, ids: &[i64]) -> Result<(), String> {
    if ids.is_empty() { return Ok(()); }
    let placeholders: Vec<String> = (0..ids.len()).map(|i| format!("?{}", i + 1)).collect();
    let sql = format!("DELETE FROM password_history WHERE id IN ({})", placeholders.join(","));
    let params: Vec<&dyn rusqlite::types::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::types::ToSql).collect();
    conn.execute(&sql, params.as_slice())
        .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn get_preference(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    let res = conn.query_row(
        "SELECT value FROM app_preferences WHERE key = ?1",
        params![key],
        |row| row.get(0),
    );
    match res {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn set_preference(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO app_preferences (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| format!("{e}"))?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PasswordEntry {
    pub id: i64,
    pub category_id: Option<i64>,
    pub label: String,
    pub username: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mode: String,
    pub template_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub pattern: String,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PasswordDisplay {
    pub id: i64,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub label: String,
    pub username: String,
    pub password: String,
    pub mode: String,
    pub template_id: Option<i64>,
    pub is_favorite: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MasterConfig {
    pub salt1: Vec<u8>,
    pub salt2: Vec<u8>,
    pub verification_hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BiometricConfig {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

pub fn init_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS master_config (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            salt BLOB NOT NULL,
            salt2 BLOB NOT NULL,
            verification_hash BLOB NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            icon TEXT NOT NULL DEFAULT 'key',
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS password_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category_id INTEGER,
            label TEXT NOT NULL,
            username TEXT NOT NULL DEFAULT '',
            ciphertext BLOB NOT NULL,
            nonce BLOB NOT NULL,
            mode TEXT NOT NULL,
            template_id INTEGER,
            is_favorite INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
        );
        CREATE TABLE IF NOT EXISTS templates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            pattern TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS biometric_config (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            ciphertext BLOB NOT NULL,
            nonce BLOB NOT NULL,
            salt BLOB NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS app_preferences (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS clipboard_expiry (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            expiry_time INTEGER NOT NULL DEFAULT 30
        );
        INSERT OR IGNORE INTO clipboard_expiry (id, expiry_time) VALUES (1, 30);",
    )
    .map_err(|e| format!("DB init failed: {e}"))?;

    // Migration: add is_favorite column if upgrading from older version
    let _ = conn.execute(
        "ALTER TABLE password_history ADD COLUMN is_favorite INTEGER NOT NULL DEFAULT 0",
        [],
    );

    Ok(())
}

pub fn is_initialized(conn: &Connection) -> Result<bool, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM master_config", [], |row| row.get(0))
        .unwrap_or(0);
    Ok(count > 0)
}

pub fn get_master_config(conn: &Connection) -> Result<Option<MasterConfig>, String> {
    let res = conn.query_row(
        "SELECT salt, salt2, verification_hash FROM master_config WHERE id = 1",
        [],
        |row| {
            Ok(MasterConfig {
                salt1: row.get(0)?,
                salt2: row.get(1)?,
                verification_hash: row.get(2)?,
            })
        },
    );
    match res {
        Ok(cfg) => Ok(Some(cfg)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn save_master_config(
    conn: &Connection,
    salt1: &[u8],
    salt2: &[u8],
    hash: &[u8],
) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO master_config (id, salt, salt2, verification_hash) VALUES (1, ?1, ?2, ?3)",
        params![salt1, salt2, hash],
    )
    .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn delete_master_config(conn: &Connection) -> Result<(), String> {
    conn.execute("DELETE FROM master_config", [])
        .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn save_password(
    conn: &Connection,
    key: &[u8; 32],
    label: &str,
    username: &str,
    password: &str,
    mode: &str,
    category_id: Option<i64>,
    template_id: Option<i64>,
) -> Result<i64, String> {
    let (ciphertext, nonce) = crypto::encrypt(key, password.as_bytes())?;
    conn.execute(
        "INSERT INTO password_history (category_id, label, username, ciphertext, nonce, mode, template_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![category_id, label, username, ciphertext, nonce, mode, template_id],
    )
    .map_err(|e| format!("{e}"))?;
    Ok(conn.last_insert_rowid())
}

pub fn get_passwords(
    conn: &Connection,
    key: &[u8; 32],
    category_id: Option<i64>,
    search: Option<&str>,
) -> Result<Vec<PasswordDisplay>, String> {
    let query = match (category_id, search) {
        (Some(_), Some(_)) => {
            "SELECT ph.id, ph.category_id, c.name, ph.label, ph.username, ph.ciphertext, ph.nonce, ph.mode, ph.template_id, ph.is_favorite, ph.created_at FROM password_history ph LEFT JOIN categories c ON ph.category_id = c.id WHERE ph.category_id = ?1 AND (ph.label LIKE ?2 OR ph.username LIKE ?2) ORDER BY ph.is_favorite DESC, ph.created_at DESC"
        }
        (Some(_), None) => {
            "SELECT ph.id, ph.category_id, c.name, ph.label, ph.username, ph.ciphertext, ph.nonce, ph.mode, ph.template_id, ph.is_favorite, ph.created_at FROM password_history ph LEFT JOIN categories c ON ph.category_id = c.id WHERE ph.category_id = ?1 ORDER BY ph.is_favorite DESC, ph.created_at DESC"
        }
        (None, Some(_)) => {
            "SELECT ph.id, ph.category_id, c.name, ph.label, ph.username, ph.ciphertext, ph.nonce, ph.mode, ph.template_id, ph.is_favorite, ph.created_at FROM password_history ph LEFT JOIN categories c ON ph.category_id = c.id WHERE ph.label LIKE ?1 OR ph.username LIKE ?1 ORDER BY ph.is_favorite DESC, ph.created_at DESC"
        }
        (None, None) => {
            "SELECT ph.id, ph.category_id, c.name, ph.label, ph.username, ph.ciphertext, ph.nonce, ph.mode, ph.template_id, ph.is_favorite, ph.created_at FROM password_history ph LEFT JOIN categories c ON ph.category_id = c.id ORDER BY ph.is_favorite DESC, ph.created_at DESC"
        }
    };

    let mut stmt = conn.prepare(query).map_err(|e| format!("{e}"))?;

    let rows = match (category_id, search) {
        (Some(cid), Some(s)) => {
            let pattern = format!("%{s}%");
            stmt.query_map(params![cid, pattern], map_password_row(key))
                .map_err(|e| format!("{e}"))?
                .collect::<Result<Vec<_>, _>>()
        }
        (Some(cid), None) => stmt
            .query_map(params![cid], map_password_row(key))
            .map_err(|e| format!("{e}"))?
            .collect::<Result<Vec<_>, _>>(),
        (None, Some(s)) => {
            let pattern = format!("%{s}%");
            stmt.query_map(params![pattern], map_password_row(key))
                .map_err(|e| format!("{e}"))?
                .collect::<Result<Vec<_>, _>>()
        }
        (None, None) => stmt
            .query_map([], map_password_row(key))
            .map_err(|e| format!("{e}"))?
            .collect::<Result<Vec<_>, _>>(),
    }
    .map_err(|e| format!("{e}"))?;

    Ok(rows)
}

fn map_password_row(
    key: &[u8; 32],
) -> impl Fn(&rusqlite::Row) -> rusqlite::Result<PasswordDisplay> + use<> {
    let k = *key;
    move |row: &rusqlite::Row| {
        let ciphertext: Vec<u8> = row.get(5)?;
        let nonce: Vec<u8> = row.get(6)?;
        let password = crypto::decrypt(&k, &ciphertext, &nonce)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, e),
            )))?;
        let password_str =
            String::from_utf8(password).unwrap_or_else(|_| "[解码失败]".to_string());
        Ok(PasswordDisplay {
            id: row.get(0)?,
            category_id: row.get(1)?,
            category_name: row.get(2)?,
            label: row.get(3)?,
            username: row.get(4)?,
            password: password_str,
            mode: row.get(7)?,
            template_id: row.get(8)?,
            is_favorite: row.get::<_, i32>(9)? != 0,
            created_at: row.get(10)?,
        })
    }
}

pub fn delete_password(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM password_history WHERE id = ?1", params![id])
        .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn get_categories(conn: &Connection) -> Result<Vec<Category>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name, icon, created_at FROM categories ORDER BY name")
        .map_err(|e| format!("{e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| format!("{e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("{e}"))?;
    Ok(rows)
}

pub fn create_category(conn: &Connection, name: &str, icon: &str) -> Result<Category, String> {
    conn.execute(
        "INSERT INTO categories (name, icon) VALUES (?1, ?2)",
        params![name, icon],
    )
    .map_err(|e| format!("{e}"))?;
    let id = conn.last_insert_rowid();
    Ok(Category {
        id,
        name: name.to_string(),
        icon: icon.to_string(),
        created_at: String::new(),
    })
}

pub fn delete_category(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM categories WHERE id = ?1", params![id])
        .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn get_templates(conn: &Connection) -> Result<Vec<Template>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name, pattern, created_at FROM templates ORDER BY created_at DESC")
        .map_err(|e| format!("{e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                pattern: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| format!("{e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("{e}"))?;
    Ok(rows)
}

pub fn save_template(conn: &Connection, name: &str, pattern: &str) -> Result<Template, String> {
    conn.execute(
        "INSERT INTO templates (name, pattern) VALUES (?1, ?2)",
        params![name, pattern],
    )
    .map_err(|e| format!("{e}"))?;
    let id = conn.last_insert_rowid();
    Ok(Template {
        id,
        name: name.to_string(),
        pattern: pattern.to_string(),
        created_at: String::new(),
    })
}

pub fn delete_template(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM templates WHERE id = ?1", params![id])
        .map_err(|e| format!("{e}"))?;Ok(())
}

pub fn update_template(conn: &Connection, id: i64, name: &str, pattern: &str) -> Result<(), String> {
    conn.execute(
        "UPDATE templates SET name = ?1, pattern = ?2 WHERE id = ?3",
        params![name, pattern, id],
    )
    .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn save_biometric_config(conn: &Connection, ciphertext: &[u8], nonce: &[u8], salt: &[u8]) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO biometric_config (id, ciphertext, nonce, salt) VALUES (1, ?1, ?2, ?3)",
        params![ciphertext, nonce, salt],
    )
    .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn delete_biometric_config(conn: &Connection) -> Result<(), String> {
    conn.execute("DELETE FROM biometric_config", [])
        .map_err(|e| format!("{e}"))?;
    Ok(())
}

pub fn has_biometric_config(conn: &Connection) -> Result<bool, String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM biometric_config", [], |row| row.get(0))
        .unwrap_or(0);
    Ok(count > 0)
}

pub fn get_biometric_config(conn: &Connection) -> Result<Option<BiometricConfig>, String> {
    let res = conn.query_row(
        "SELECT ciphertext, nonce, salt FROM biometric_config WHERE id = 1",
        [],
        |row| {
            Ok(BiometricConfig {
                ciphertext: row.get(0)?,
                nonce: row.get(1)?,
                salt: row.get(2)?,
            })
        },
    );
    match res {
        Ok(cfg) => Ok(Some(cfg)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(format!("{e}")),
    }
}
