use std::{
    hash::Hash,
};

mod indirect;
pub mod key;

pub use indirect::Indirect;

use super::service::ID as id;

/**
 * An trait that can be used to delay the creation of a database entry.
 */
pub trait Creator: Hash {
    /// The result of creation.
    type Entry;
    /// A possible error from creation.
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
    #[error("no entry not found with key{0}")]
    EntryNotFound(Box<dyn std::fmt::Debug>),
}

impl Into<id> for Error {
    fn into(self) -> id {
        match self {
            Self::EntryNotFound(_) => 1,
        }
    }
}

pub trait Like {
    type Key;
    type Value;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    /**
     * A convenience function to map an empty `Option` to an
     * error.
     */
    fn must_get(&self, key: &Self::Key) -> Result<&Self::Value, Error> {
        self.get(key).ok_or(Error::EntryNotFound(key))
    }
    ///
    /// Insert an entry to the database.
    fn insert(&mut self, key: &Self::Key, entry: Self::Value);

    /// Remove an entry from the database.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /**
     * A convenience function to map an empty `Option` to an
     * error.
     */
    fn must_remove(&mut self, key: &Self::Key) -> Result<Self::Value, Error> {
        self.remove(key).ok_or(Error::EntryNotFound(key))
    }

    /// Check if an entry is contained in the database.
    fn contains(&self, key: &Self::Key) -> bool;
}


pub trait Generic<K, V> : Like<Key = K, Value = V> {}

/// Specialized version of the `Generic` trait with `ID` used as key.
pub trait ID<Entry> : Generic<id, Entry> {}

/**
 * A subtype of `ID` that can calculate the `ID` of an entry
 * based on its hash value.
 */
pub trait AutoHash<Entry>: ID<Entry> {
    fn insert_auto(&mut self, entry: Entry) -> id;
}
