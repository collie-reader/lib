#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unauthorized")]
    Unauthorized,

    #[error(transparent)]
    RusqliteError {
        #[from]
        source: rusqlite::Error,
    },

    #[error(transparent)]
    SeaQueryError {
        #[from]
        source: sea_query::error::Error,
    },

    #[error(transparent)]
    JsonWebToken {
        #[from]
        source: jsonwebtoken::errors::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
