use std::{
    error,
};


pub mod database;
pub mod service;
pub mod string;
mod bindings;

pub use database::{Database};
pub use service::{CResult, ID};

pub trait Error : error::Error + Into<service::ID> {}
