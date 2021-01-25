use std::{
    hash::Hash,
};

mod unique;
pub mod indirect;
pub mod key;

pub use indirect::Indirect;
pub use unique::Unique;

use super::service::ID as service_id;

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

pub trait CError : std::error::Error + Into<service_id> {}


/**
 * Aggregate error for this module. At the moment it only contains
 * the error for missing database entry.
 */
#[derive(thiserror::Error, Debug)]
pub enum Error<T: std::fmt::Debug> {
    #[error("no entry not found with key {0}")]
    EntryNotFound(T),
}

impl<T> Into<service_id> for Error<T> {
    fn into(self) -> service_id {
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
    fn must_get(&self, key: &Self::Key) -> Result<&Self::Value, Error<Self::Key>> {
        self.get(key).ok_or(Error::EntryNotFound(key))
    }

    /// Insert an entry to the database.
    fn insert(&mut self, key: &Self::Key, entry: Self::Value);

    /// Remove an entry from the database.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /**
     * A convenience function to map an empty `Option` to an
     * error.
     */
    fn must_remove(&mut self, key: &Self::Key) -> Result<Self::Value, Error<Self::Key>> {
        self.remove(key).ok_or(Error::EntryNotFound(key))
    }

    /// Check if an entry is contained in the database.
    fn contains(&self, key: &Self::Key) -> bool;
}

/**
 * Variant of databases that use `Type` (alies `u64`) as key.
 */
pub mod id {
    pub type Type = super::service_id;
    pub type Indirect<KC, V, DB, C> = super::Indirect<KC, Type, V, DB, C>;

    /// Specialized version of the `Generic` trait with `ID::Type` used as key.
    pub trait Generic<V> : super::Generic<Type, V> {}
}

pub trait Generic<K, V> : Like<Key = K, Value = V> {}


/**
 * A subtype of `ID` that can calculate the `ID` of an entry
 * based on its hash value.
 */
pub trait AutoHash<Entry> : id::Generic<Entry> {
    fn insert_auto(&mut self, entry: Entry) -> id::Type;
}
