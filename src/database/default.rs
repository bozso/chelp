use std::{
    default::Default as StdDefault,
    hash::{Hash, BuildHasher},
    collections::hash_map::RandomState,
};

use crate::{
    database::{
        Base, Database
    },
    service,
};

#[derive(Debug)]
pub struct Default<T, S = RandomState> {
    base: Base<T, S>,
}

impl<T: Hash> StdDefault for Default<T, RandomState> {
    fn default() -> Self {
        Self {
            base: Base::new(RandomState::new())
        }
    }
}

impl<T: Hash, B: BuildHasher> Database for Default<T, B> {
    type Entry = T;
    
    fn insert(&mut self, entry: Self::Entry) -> service::ID {
        self.base.insert(entry)
    }
    
    fn get(&self, id: service::ID) -> Option<&Self::Entry> {
        self.base.get(id)
    }

    fn remove(&mut self, id: service::ID) {
        self.base.remove(id)
    }
}
