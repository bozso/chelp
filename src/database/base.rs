use std::{
    collections::HashMap,
    //hash::{Hasher, Hash, BuildHasher},
};

use crate::{
    database::Database,
    service,
};

#[derive(Debug)]
pub struct Base<T> {
    db: HashMap<u64, T>,
}

impl<T> Base<T> {
    pub fn new() -> Self {
        Self {
            db: HashMap::<u64, T>::new(),
        }
    }
}

impl<T> Database for Base<T> {
    type Entry = T;
    
    fn insert(&mut self, id: service::ID, entry: Self::Entry) {
        self.db.insert(id, entry);
    }
    
    fn remove(&mut self, id: service::ID) {
        self.db.remove(&id);
    }
    
    fn get(&self, id: service::ID) -> Option<&Self::Entry> {
        self.db.get(&id)
    }

    fn contains(&self, id: service::ID) -> bool {
        self.db.contains_key(&id)
    }
}
