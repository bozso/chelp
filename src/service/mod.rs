use std::{
    //hash::Hash,
    default::Default,
};

use crate::{
    Result as LibResult,
    string,
    database::{default, Database},
};


mod cservice;
mod cresult;

pub use cservice::{CService, ID};
pub use cresult::{CResult, CStatus};

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("entry not found")]
    EntryNotFound{
        id: ID,
    },
}

impl Into<ID> for Error {
    fn into(self) -> ID {
        match self {
            Self::EntryNotFound{id: _} => 1,
        }
    }
}

pub type Result<T> = LibResult<T, Error>;

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

pub struct Service<DBS: Database<Entry = String>> {
    string_service: string::Service<DBS>,
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

impl<DBS: Database<Entry = String>> Service<DBS> {
    pub fn new(dbs: DBS) -> Self {
        Self {
            string_service: string::Service::new(dbs),
        }
    }
}

pub type DefaultService = Service<default::Default::<String>>;

impl Default for DefaultService {
    fn default() -> Self {
        Self::new(default::Default::<String>::default())
    }
    
}

/*
impl<default::Default::<String>> Default for Service<default::Default::<String>> {
    fn default() -> Self {
        Self::new(default::Default::<String>::default())
    }
}
*/
