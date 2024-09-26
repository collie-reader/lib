use collie_core::repository::database::DbConnection;
use rand::{thread_rng, Rng};

use crate::error::Result;
use crate::model::key::KeyToCreate;
use crate::repository::key;

pub fn create(conn: DbConnection, description: Option<&str>) -> Result<(String, String)> {
    let access_key = generate();
    let secret_key = generate();

    let _ = key::create(
        &conn,
        &KeyToCreate {
            access: access_key.clone(),
            secret: secret_key.clone(),
            description: description.map(|x| x.to_string()),
            expired_at: None,
        },
    );

    Ok((access_key, secret_key))
}

pub fn generate() -> String {
    const CHARS: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_-";

    let mut rng = thread_rng();
    (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
}
