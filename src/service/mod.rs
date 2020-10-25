use std::{
    hash::Hash,
};

use crate::{
    Result as LibResult,
    string,
    //database::{Database, Default},
    database::{default, Maker, Database},
    //database::Database,
    //database::{Maker, Database},
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


pub enum Services<T: Hash> {
    phantom(PhantomData<T>),
    Default(Service<T, default::Default<T>>),
}

impl<T: Hash> Services<T> {
    fn default() -> Self {
        Self::Default(Service::new(&default::Maker::new()))
    }
}

pub struct Service<T, DB: Database<Entry=T>> {
    string_service: string::Service<DB>,
}

impl<T, DB: Database<Entry=T>> Service<T, DB> {
    pub fn new<M: Maker<T, DB=DB>>(maker: &M) -> Self {
        Self {
            string_service: string::Service::new(maker.make()),
        }
    }
}
