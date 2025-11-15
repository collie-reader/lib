use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use collie_core::repository::database::DbConnection;
use rand::{thread_rng, Rng};

use crate::error::{Error, Result};
use crate::model::key::KeyToCreate;
use crate::repository::key;

pub fn create(conn: DbConnection, description: Option<&str>) -> Result<(String, String)> {
    let access_key = generate();
    let secret_key = generate();
    let hashed_secret = hash_secret(&secret_key)?;

    let _ = key::create(
        &conn,
        &KeyToCreate {
            access: access_key.clone(),
            secret: hashed_secret,
            description: description.map(|x| x.to_string()),
            expired_at: None,
        },
    );

    Ok((access_key, secret_key))
}

pub fn create_with_credentials(
    conn: DbConnection,
    access: &str,
    secret: &str,
    description: Option<&str>,
) -> Result<()> {
    let hashed_secret = hash_secret(secret)?;

    let _ = key::create(
        &conn,
        &KeyToCreate {
            access: access.to_string(),
            secret: hashed_secret,
            description: description.map(|x| x.to_string()),
            expired_at: None,
        },
    );

    Ok(())
}

pub fn hash_secret(secret: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(secret.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| Error::Internal)
}

pub fn verify_secret(secret: &str, hashed: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hashed).map_err(|_| Error::Internal)?;
    Ok(Argon2::default()
        .verify_password(secret.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate() -> String {
    const CHARS: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_-";

    let mut rng = thread_rng();
    (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
}
