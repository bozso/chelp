use crate::{
    Result as LibResult,
    string,
    //database::{Database, Default},
    //database::{default, Maker, Database},
    database::Database,
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
pub enum Services<T> {
    Default(Service<default::Default<T>>),
}

impl<T> Services<T> {
    fn default() -> Self {
        Self::Default(Service::new(default::Maker::new()))
    }
}
*/

pub struct Service<T, DB: Database<Entry=T>> {
    string_service: string::Service<DB>,
}

/*
impl<DB> Service<DB> {
    pub fn new<T, M: Maker<T>>(maker: &M) -> Self {
        Self {
            string_service: string::Service::new(maker.make()),
        }
    }
}
*/
