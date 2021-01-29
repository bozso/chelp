use crate::{
    service
};

#[derive(thiserror::Error, Debug)]
pub enum Error<T: std::fmt::Debug> {
    #[error("failed to lock service structure")]
    LockFail,
    #[error("error from service: {0}")]
    Service(#[from] service::Error<T>),
}

/**
 * \TODO: properly handle different error types
 */
impl<T> Into<service::ID> for Error<T> {
    fn into(self) -> service::ID {
        3
    }
}

impl<T> crate::Error for Error<T> {}

