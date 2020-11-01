use crate::{
    Error as LibError,
    string,
    service
};

//pub struct LockFail;

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("failed to lock service structure")]
    LockFail,
    #[error("error while managing strings: {0}")]
    String(#[from] string::Error),
    #[error("io error occurred: {0}")]
    IOError(#[from] std::io::Error),
    #[error("io error occurred: {0}")]
    CIOError(#[from] crate::io::Error),
}

/**
 * \TODO: properly handle different error types
 */
impl Into<service::ID> for Error {
    fn into(self) -> service::ID {
        3
    }
}

impl LibError for Error {}

