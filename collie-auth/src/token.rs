use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use collie_core::model::database::DbConnection;

use crate::error::{Error, Result};
use crate::model;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Login {
    pub access: String,
    pub secret: String,
}

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
    let exists = model::key::exists(conn, access, secret)?;
    if exists {
        Ok(encode(server_secret)?)
    } else {
        Err(Error::Unauthorized)
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
