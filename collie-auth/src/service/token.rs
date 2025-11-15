use chrono::Utc;
use collie_core::repository::database::DbConnection;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::error::{Error, Result};
use crate::model::token::Claims;
use crate::repository::key;
use crate::service::key as key_service;

pub fn verify(access: &str, server_secret: &str) -> Result<bool> {
    let validation = Validation::default();
    match jsonwebtoken::decode::<Claims>(
        access,
        &DecodingKey::from_secret(server_secret.as_bytes()),
        &validation,
    ) {
        Ok(token) => {
            if token.claims.exp > Utc::now().timestamp() {
                Ok(true)
            } else {
                Err(Error::Unauthorized)
            }
        }
        Err(_) => Err(Error::Unauthorized),
    }
}

pub fn issue(
    conn: &DbConnection,
    access: &str,
    secret: &str,
    server_secret: &str,
) -> Result<String> {
    let hashed_secret = key::get_hashed_secret(conn, access)?;
    match hashed_secret {
        Some(hashed) => {
            if key_service::verify_secret(secret, &hashed)? {
                Ok(encode(server_secret)?)
            } else {
                Err(Error::Unauthorized)
            }
        }
        None => Err(Error::Unauthorized),
    }
}

fn encode(secret: &str) -> Result<String> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        iat: now,
        exp: now + 3600,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| Error::Unauthorized)
}
