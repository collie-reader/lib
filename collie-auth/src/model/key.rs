use chrono::{DateTime, FixedOffset};
use rusqlite::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Key {
    pub id: i32,
    pub access: String,
    pub secret: String,
    pub description: Option<String>,
    pub expired_at: Option<DateTime<FixedOffset>>,
}

impl From<&Row<'_>> for Key {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            access: row.get_unwrap("access"),
            secret: row.get_unwrap("secret"),
            description: row.get_unwrap("description"),
            expired_at: row.get_unwrap("expired_at"),
        }
    }
}

#[derive(Deserialize)]
pub struct KeyToCreate {
    pub access: String,
    pub secret: String,
    pub description: Option<String>,
    pub expired_at: Option<DateTime<FixedOffset>>,
}
