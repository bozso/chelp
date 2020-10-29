use std::{
    //hash::Hash,
    default::Default,
};

use crate::{
    string,
    io,
    database::{default, Database},
};


mod cservice;
mod cresult;

pub use cservice::{CService, ID};
pub use cresult::{CResult, CStatus};

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("entry not found {0}")]
    EntryNotFound(ID),
}

impl Into<ID> for Error {
    fn into(self) -> ID {
        match self {
            Self::EntryNotFound(_) => 1,
        }
    }
}

/*
pub enum Services {
    Default(Service<default::Default>),
}

impl<T: Hash> Services<T> {
    fn default() -> Self {
        Self::Default(Service::new(&default::Maker::new()))
    }
}
*/

#[derive(Debug)]
pub struct Service<S, F>
where
    S: Database<Entry = String>,
    F: Database<Entry = std::fs::File>,
{
    pub string_service: string::Service<S>,
    pub file_service: io::Service<F>,
}

/*
impl<DB: Database> Service<DB> {
    pub fn new(makers: &Makers<DB>) -> Self {
        Self {
            string_service: string::Service::new(makers.string.make()),
        }
    }
}
*/

impl<S, F> Service<S, F>
where
    S: Database<Entry = String>,
    F: Database<Entry = std::fs::File>,
{
    pub fn new(s: S, f: F) -> Self {
        Self {
            string_service: string::Service::new(s),
            file_service: io::Service::new(f),
        }
    }
}

pub type DefaultService = Service<
    default::Default::<String>,
    default::Default::<std::fs::File>,
>;

impl Default for DefaultService {
    fn default() -> Self {
        Self::new(
            default::Default::<String>::default(),
            default::Default::<std::fs::File>::default()
        )
    }
}

/*
impl<default::Default::<String>> Default for Service<default::Default::<String>> {
    fn default() -> Self {
        Self::new(default::Default::<String>::default())
    }
}
*/
