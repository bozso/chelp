use std::{
    hash::{Hash, Hasher, BuildHasher},
    collections::hash_map::RandomState,
};

use crate::{
    database::{
        Base, Database, AutoHash,
    },
    service,
};

#[derive(Debug)]
pub struct Default<T, S = RandomState> {
    builder: S,
    base: Base<T>,
}

impl<T: Hash> std::default::Default for Default<T, RandomState> {
    fn default() -> Self {
        Self {
            builder: RandomState::new(),
            base: Base::new()
        }
    }
}

impl<T: Hash, B: BuildHasher> Default<T, B> {
    fn calc_id(&self, entry: &T) -> u64 {
        let mut hasher = self.builder.build_hasher();
        entry.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T, B: BuildHasher> Database for Default<T, B> {
    type Entry = T;
    
    fn insert(&mut self, id: service::ID, entry: Self::Entry) {
        self.base.insert(id, entry)
    }
    
    fn get(&self, id: service::ID) -> Option<&Self::Entry> {
        self.base.get(id)
    }

    fn remove(&mut self, id: service::ID) -> Option<Self::Entry>{
        self.base.remove(id)
    }
    
    fn contains(&self, id: service::ID) -> bool {
        self.base.contains(id)
    }
}

impl<T: Hash, B: BuildHasher> AutoHash for Default<T, B> {
    fn insert_auto(&mut self, entry: Self::Entry) -> u64 {
        let id = self.calc_id(&entry);

        self.base.insert(id, entry);
        id
    }
}
