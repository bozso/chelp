use std::{
    hash::Hash,
};

mod base;
pub mod default;

pub use base::Base;

use crate::service::ID;

pub trait Database {
    type Entry;
    
    fn get(&self, id: ID) -> Option<&Self::Entry>;
    fn insert(&mut self, entry: Self::Entry) -> ID;
    fn remove(&mut self, id: ID);
}

pub trait Type<T> {
    type DB: Database<Entry = T>;
}

pub trait Maker {
    fn make<T: Hash, DB: Database<Entry = T>>(&self) -> DB;
}
