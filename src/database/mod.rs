//use std::{
    //hash::Hash,
//};

mod base;
pub mod default;

pub use base::Base;

pub trait Database {
    type Entry;
    
    fn put(&mut self, entry: Self::Entry);
    fn get(&self, id: u64) -> Option<&Self::Entry>;
}

pub trait Type<T> {
    type DB: Database<Entry = T>;
}

pub trait Maker<T, DB> {
    //type DB: Database<Entry = T>;
    fn make(&self) -> DB;
}
