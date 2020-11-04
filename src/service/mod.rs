mod error;
mod io;
mod string;
mod service;
mod cresult;

pub type ID = u64;
pub type Result<T> = std::result::Result<T, Error>;

pub use cresult::{CResult, CStatus};
pub use service::{Service, Default};
pub use error::Error;
