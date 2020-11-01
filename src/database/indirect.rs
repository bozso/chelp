use std::{
    marker::PhantomData,
    hash::{BuildHasher, Hasher, Hash},
    collections::hash_map::RandomState,
};

use crate::{
    database::{Database, Creator},
    service::ID,
};

#[derive(Debug)]
pub struct Indirect<T, B, DB> {
    builder: B,
    db: DB,
    t: PhantomData<T>,
}

impl<T, B, DB> Indirect<T, B, DB>
where
    B: BuildHasher,
{
    pub fn calc_id<H: Hash>(&self, h: &H) -> u64 {
        let mut hasher = self.builder.build_hasher();
        h.hash(&mut hasher);
        hasher.finish()
    }
}

impl<T, B, DB> Indirect<T, B, DB>
where
    B: BuildHasher,
    DB: Database<Entry = T>
{
    pub fn lazy_insert<C: Creator<Entry = T>>(&mut self, c: &C) -> Result<u64, <C as Creator>::Error> {
        let id = self.calc_id(c);
        
        if !self.db.contains(id) {
            self.db.insert(id, c.create()?)
        }
        
        Ok(id)
    }
}

impl<T, B, DB> Database for Indirect<T, B, DB>
where
    B: BuildHasher,
    DB: Database<Entry = T>
{
    type Entry = T;

    fn get(&self, id: ID) -> Option<&Self::Entry> {
        self.db.get(id)
    }

    fn insert(&mut self, id: ID, entry: Self::Entry) {
        self.db.insert(id, entry)
    }
    
    fn remove(&mut self, id: ID) -> Option<Self::Entry> {
        self.db.remove(id)
    }
    
    fn contains(&self, id: ID) -> bool {
        self.db.contains(id)
    }
}

pub type Default<T, DB> = Indirect<T, RandomState, DB>;

impl<T, B, DB> Indirect<T, B, DB> {
    pub fn new(b: B, db: DB) -> Self {
        Self {
            builder: b,
            db: db,
            t: PhantomData,            
        }
    }
}

impl<T, DB: Database> Indirect<T, RandomState, DB> {
    pub fn with_db(db: DB) -> Self {
        Self::new(RandomState::new(), db)
    }
}
