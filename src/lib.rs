pub mod database;
pub mod service;
pub mod bindings;

pub use database::{Database};
pub use service::{CResult, ID};

pub trait Error : std::error::Error + Into<service::ID> {}
