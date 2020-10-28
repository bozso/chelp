use std::{
    collections::HashMap,
    hash::{Hasher, Hash, BuildHasher},
};

use crate::{
    database::Database,
    service,
};

#[derive(Debug)]
pub struct Base<T, B> {
    builder: B,
    db: HashMap<u64, T>,
}

impl<T: Hash, B> Base<T, B> {
    pub fn new(builder: B) -> Self {
        Self {
            builder,
            db: HashMap::<u64, T>::new(),
        }
    }
}

impl<T: Hash, B: BuildHasher> Base<T, B> {
    fn calc_id(&self, entry: &T) -> u64 {
        let mut hasher = self.builder.build_hasher();
        entry.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: Hash, B: BuildHasher> Database for Base<T, B> {
    type Entry = T;
    
    fn insert(&mut self, entry: Self::Entry) -> service::ID {
        let id = self.calc_id(&entry);
        self.db.insert(id, entry);
        id
    }
    
    fn remove(&mut self, id: service::ID) {
        self.db.remove(&id);
    }
    
    fn get(&self, id: service::ID) -> Option<&Self::Entry> {
        self.db.get(&id)
    }
    
}
