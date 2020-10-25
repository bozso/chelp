use std::{
    hash::Hash,
};

mod base;
pub mod default;

pub use base::Base;

pub trait Database {
    type Entry;
    fn put(&mut self, entry: Self::Entry) -> u64;
    fn get(&self, id: u64) -> Option<&Self::Entry>;
}

pub trait Type<T> {
    type DB: Database<Entry = T>;
}

pub trait Maker {
    fn make<T: Hash, DB: Database<Entry = T>>(&self) -> DB;
}
