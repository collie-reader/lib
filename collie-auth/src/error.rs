#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,

    #[error("failed to ")]
    JsonWebToken {
        #[from]
        source: jsonwebtoken::errors::Error,
    },

    #[error(transparent)]
    CoreError {
        #[from]
        source: collie_core::error::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
