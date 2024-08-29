use collie_core::model::database::Connection;
use rand::{thread_rng, Rng};

use crate::error::Result;
use crate::model;
use crate::model::key::KeyToCreate;

pub fn create(conn: Connection, description: &Option<String>) -> Result<String> {
    let access_key = generate_key();
    let secret_key = generate_key();

    let _ = model::key::create(
        &conn,
        &KeyToCreate {
            access: access_key.clone(),
            secret: secret_key,
            description: description.clone(),
            expired_at: None,
        },
    );

    Ok(access_key)
}

fn generate_key() -> String {
    const TOKEN_CHARS: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_-";

    let mut rng = thread_rng();
    (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..TOKEN_CHARS.len());
            TOKEN_CHARS[idx] as char
        })
        .collect()
}
