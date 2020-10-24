use std::{
    hash::Hasher
};

mod database;

pub use database::Default;

pub trait HashMaker<H: Hasher> {
    fn make(&self) -> H;
}

pub trait Database {
    type Entry;
    
    fn put(& mut self, entry: Self::Entry);
    fn get(&self, id: u64) -> Option<&Self::Entry>;
}
