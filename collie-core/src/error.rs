use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid key `{0}` for `{1}`")]
    InvalidEnumKey(String, String),

    #[error("failed to parse syndication feed")]
    SyndicationParsingFailure,

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
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error(transparent)]
    IoError {
        #[from]
        source: io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
