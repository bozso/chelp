mod error;
mod io;
mod string;
mod service;
mod cresult;

pub type ID = u64;
pub type Result<T, K> = std::result::Result<T, Error<K>>;

pub use cresult::{CResult, CStatus};
//pub use service::{Service, Default};
pub use error::Error;
