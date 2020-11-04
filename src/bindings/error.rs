use crate::{
    service
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to lock service structure")]
    LockFail,
    #[error("error from service: {0}")]
    Service(#[from] service::Error),
}

/**
 * \TODO: properly handle different error types
 */
impl Into<service::ID> for Error {
    fn into(self) -> service::ID {
        3
    }
}

impl crate::Error for Error {}

