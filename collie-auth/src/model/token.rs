use serde::{Deserialize, Serialize};

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
