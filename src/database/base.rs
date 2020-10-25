use std::{
    collections::HashMap,
    hash::{Hasher, Hash, BuildHasher},
};

use crate::database::Database;

pub struct Base<T, B> {
    builder: B,
    db: HashMap<u64, T>,
}

impl<T: Hash, B> Base<T, B> {
    pub fn new(builder: B) -> Self {
        Self {
            builder: builder,
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
    
    fn put(&mut self, entry: Self::Entry) -> Result<u64> {
        let id = self.calc_id(&entry);
        self.db.insert(id, entry);
        id
    }
    
    fn get(&self, id: u64) -> Option<&Self::Entry> {
        self.db.get(&id)
    }
    
}
