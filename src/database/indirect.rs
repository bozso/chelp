use std::{
    marker::PhantomData,
};

use crate::{
    database as db,
};

#[derive(Debug)]
pub struct Indirect<KC, K, V, DB, C> {
    key_calculator: KC,
    db: DB,
    t: PhantomData<(K, V, C)>,
}

impl<KC, K, V, DB, C> Indirect<KC, K, V, DB, C>
where
    K: std::fmt::Debug,
    C: db::Creator<Entry = V>,
    KC: db::key::Calculator<Key = K, Value = C>,
    DB: db::Generic<K, V>
{
    pub fn lazy_insert(&mut self, c: &C) -> Result<K, C::Error> {
        let key = self.key_calculator.calc_key(c);
        
        if !self.db.contains(&key) {
            self.db.insert(&key, c.create()?)
        }
        
        Ok(key)
    }
}

impl<KC, K, V, DB, C> db::Like for Indirect<KC, K, V, DB, C>
where
    K: std::fmt::Debug,
    C: db::Creator<Entry = V>,
    KC: db::key::Calculator<Key = K, Value = C>,
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

pub type Default<K, V, DB, C> = Indirect<
    db::key::DefaultWrapHasher<C>,
    K,
    V,
    C,
    DB
>;

impl<KC, K, V, DB, C> Indirect<KC, K, V, DB, C> {
    pub fn new(key_calculator: KC, db: DB) -> Self {
        Self {
            key_calculator: key_calculator,
            db: db,
            t: PhantomData,
        }
    }
}

