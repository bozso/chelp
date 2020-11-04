use std::{
    hash::Hash,
};

mod base;
pub mod indirect;
pub mod default;

pub use base::Base;

use crate::service::ID;

/**
 * An trait that can be used to delay the creation of a database entry.
 */
pub trait Creator: Hash {
    /// The result of creation.
    type Entry;
    /// A possible error from creatio.
    type Error: std::error::Error;
    
    /// The function that creates the database entry.
    fn create(&self) -> Result<Self::Entry, Self::Error>;
}

/**
 * Aggregate error for this module. At the moment it only contains
 * the error for missing database entry.
 */
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

/**
 * 
 */
pub trait Database {
    /// The type that is stored in the database.
    type Entry;
    
    /// Get a database entry based on ID.
    fn get(&self, id: ID) -> Option<&Self::Entry>;
    
    /// Insert an entry to the database.
    fn insert(&mut self, id: ID, entry: Self::Entry);
    
    /// Remove an entry from the database.
    fn remove(&mut self, id: ID) -> Option<Self::Entry>;
    
    /// Check if an entry is contained in the database.
    fn contains(&self, id: ID) -> bool;
    
    /**
     * A convenience function to map an empty `Option` to an
     * error.
     */
    fn must_get(&self, id: ID) -> Result<&Self::Entry, Error> {
        self.get(id).ok_or(Error::EntryNotFound(id))
    }

    /**
     * A convenience function to map an empty `Option` to an
     * error.
     */
    fn must_remove(&mut self, id: ID) -> Result<Self::Entry, Error> {
        self.remove(id).ok_or(Error::EntryNotFound(id))
    }
}

/**
 * A subtype of database that can calculate the `ID` of an entry
 * based on its hash value.
 */
pub trait AutoHash: Database {
    fn insert_auto(&mut self, entry: Self::Entry) -> ID;
}
