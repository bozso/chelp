use std::{
    hash::BuildHasher,
    marker::PhantomData,
};

pub trait Calculator {
    type Key;
    type Value;

    fn calc_key(&self, v: &Self::Value) -> &Self::Key;
}

pub struct Hasher<B, T> {
    builder: B,
    phantom: PhantomData<T>,
}

impl<B, T> Hasher<B, T> {
    pub fn new(builder: B) -> Self {
        Self {
            builder: builder,
            phantom: PhantomData,
        }
    }
}

impl<B: BuildHasher, T> Calculator for Hasher<B, T> {
    type Key = u64;
    type Value = T;

    fn calc_key(&self, v: &T) -> u64 {
        let mut hasher = self.builder.build_hasher();
        v.hash(&mut hasher);
        hasher.finish()
    }
}
