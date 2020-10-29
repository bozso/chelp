use std::{
    error,
};


pub mod database;
pub mod service;
pub mod string;
pub mod bindings;
pub mod io;

pub use database::{Database};
pub use service::{CResult, ID};

pub trait Error : error::Error + Into<service::ID> {}
