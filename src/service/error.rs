use crate::service::ID;


const IO_ERROR: ID = 1;
const CONVERSION_ERROR: ID = 2;

#[derive(thiserror::Error, Debug)]
pub enum Error<T: 'static + std::fmt::Debug> {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("error while converting from pointer: {0}")]
    ConversionError(#[from] std::ffi::IntoStringError),
    #[error("database error: {0}")]
    DBError(#[from] crate::database::Error<T>),
}

impl<T: std::fmt::Debug> Into<ID> for Error<T> {
    fn into(self) -> ID {
        match self {
            Self::ConversionError(_) => CONVERSION_ERROR,
            Self::IOError(_) => IO_ERROR,
            Self::DBError(e) => e.into(),
        }
    }
}
