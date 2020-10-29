use std::{
    hash::Hash,
};

mod base;
pub mod default;

pub use base::Base;

use crate::service::ID;

pub trait Creator: Hash {
    type Result;
    
    fn create(&self) -> Self::Result;
}

pub trait Database {
    type Entry;
    
    fn get(&self, id: ID) -> Option<&Self::Entry>;
    fn insert(&mut self, entry: Self::Entry) -> ID;
    fn remove(&mut self, id: ID);
}

pub trait LazyInsert: Database {
    type Create: Creator;
    
    fn calc_id(&self, creator: &Self::Create) -> ID;
    fn contains(&self, id: ID) -> bool;
    
    fn lazy_insert(&mut self, creator: Self::Create) -> Self::Create::Result {
        let id = self.calc_id(&creator);
        
        if self.contains(id) {
            Ok(id)
        } else {
            self.insert(creator.create()?)
        }
    }
}

pub trait Type<T> {
    type DB: Database<Entry = T>;
}

pub trait Maker {
    fn make<T: Hash, DB: Database<Entry = T>>(&self) -> DB;
}
