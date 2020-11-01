use std::{
    hash::Hash,
};

mod base;
pub mod indirect;
pub mod default;

pub use base::Base;

use crate::service::ID;

pub trait Creator: Hash {
    type Entry;
    type Error: std::error::Error;
    
    fn create(&self) -> Result<Self::Entry, Self::Error>;
}

#[derive(thiserror::Error, Debug)]
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

pub trait Database {
    type Entry;
    
    fn get(&self, id: ID) -> Option<&Self::Entry>;
    fn insert(&mut self, id: ID, entry: Self::Entry);
    fn remove(&mut self, id: ID) -> Option<Self::Entry>;
    
    fn contains(&self, id: ID) -> bool;
    
    fn must_get(&self, id: ID) -> Result<&Self::Entry, Error> {
        self.get(id).ok_or(Error::EntryNotFound(id))
    }

    fn must_remove(&mut self, id: ID) -> Result<Self::Entry, Error> {
        self.remove(id).ok_or(Error::EntryNotFound(id))
    }
}

pub trait AutoHash: Database {
    fn insert_auto(&mut self, entry: Self::Entry) -> ID;
}

pub trait Type<T> {
    type DB: Database<Entry = T>;
}

pub trait Maker {
    fn make<T: Hash, DB: Database<Entry = T>>(&self) -> DB;
}
