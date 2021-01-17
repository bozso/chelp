use std::{
    marker::PhantomData,
};

use crate::{
    database as db
};

pub struct Unique<S, DB, K, V> {
    set: S,
    db: DB,
    phantom: PhantomData<(K, V)>,
}

impl<S, DB, K, V> Unique<S, DB, K, V>{
    pub fn new(set: S, db: DB) -> Self {
        Self {
            set: set,
            db: db,
            phantom: PhantomData,
        }
    }
}

impl<S, DB, K, V> db::Like for Unique<S, DB, K, V>
where
    S: db::Like<Key = V, Value = ()>,
    DB: db::Like<Key = K, Value = V>,
    V: Clone,
{
    type Key = K;
    type Value = V;

    fn get(&self, key: &K) -> Option<&V> {
        self.db.get(key)
    }

    fn insert(&mut self, key: &K, value: V) {
        if self.set.contains(&value) {
            return
        }
        self.db.insert(key, value.clone());
        self.set.insert(&value, ());
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.db.remove(key).map(|v| {
            self.set.remove(v)
        })
    }
}

