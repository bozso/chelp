use std::{
    default::Default as StdDefault,
    hash::{Hash, BuildHasher},
    collections::hash_map::RandomState,
};

use crate::database::{
    Base, Database
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
    fn put(&mut self, entry: Self::Entry) -> u64 {
        self.base.put(entry)
    }
    
    fn get(&self, id: u64) -> Option<&Self::Entry> {
        self.base.get(id)
    }
}
