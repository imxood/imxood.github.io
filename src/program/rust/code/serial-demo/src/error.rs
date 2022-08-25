use thiserror::Error as ThisError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("操作超时")]
    Timeout,

    #[error("{0}")]
    Error(String),

    #[error("Unknown error")]
    Unknown,
}
