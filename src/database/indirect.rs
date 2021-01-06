use std::{
    marker::PhantomData,
    hash::{BuildHasher, Hasher, Hash},
    collections::hash_map::RandomState,
};

use crate::{
    database as db,
    service::ID,
};

#[derive(Debug)]
pub struct Indirect<KC, K, V, DB> {
    KeyCalculator: KC,
    db: DB,
    t: PhantomData<(K, V)>,
}

impl<KC, K, V, DB> Indirect<KC, K, V, DB>
where
    KC: db::key::Calculator,
    DB: db::AutoHash<V>
{
    pub fn lazy_insert<C: db::Creator<Entry = T>>(&mut self, c: &C) -> Result<u64, <C as db::Creator>::Error> {
        let id = self.calc_id(c);
        
        if !self.db.contains(id) {
            self.db.insert(id, c.create()?)
        }
        
        Ok(id)
    }
}

impl<KC, K, V, DB> db::Like for Indirect<KC, K, V,  DB>
where
    KC: db::key::Calculator<Key = K, Value = V>,
    DB: db::Generic<K, V>
{
    type Key = K;
    type Value = V;

    fn get(&self, key: &K) -> Option<&V> {
        self.db.get(key)
    }

    fn insert(&mut self, key: &K, entry: V) {
        self.db.insert(key, entry)
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        self.db.remove(key)
    }
    
    fn contains(&self, key: &K) -> bool {
        self.db.contains(key)
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
