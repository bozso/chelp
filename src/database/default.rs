use std::{
    marker::PhantomData,
    default::Default as StdDefault,
    hash::{Hash, BuildHasher},
    collections::hash_map::RandomState,
};

use crate::database::{
    Base, Database,
    Maker as DMaker,
};

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
    
    fn put(&mut self, entry: Self::Entry) {
        self.base.put(entry);
    }
    
    fn get(&self, id: u64) -> Option<&Self::Entry> {
        self.base.get(id)
    }
}

pub struct Maker<T> {
    phantom: PhantomData<T>,
}

impl<T> Maker<T> {
    pub fn new() -> Self { Self{ phantom: PhantomData} }
}

impl<T: Hash> DMaker<T, Default<T>> for Maker<T> {
    //type DB = Default<T>;
    
    fn make(&self) -> Default<T> {
        Default::<T>::default()
    }
}
