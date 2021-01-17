use std::{
    hash::{BuildHasher, Hash, Hasher},
    collections::hash_map::{DefaultHasher, RandomState},
    marker::PhantomData,
};

pub trait Calculator {
    type Key;
    type Value;

    fn calc_key(&self, v: &Self::Value) -> Self::Key;
}

pub struct WrapHasher<H, B, T> {
    builder: B,
    phantom: PhantomData<(H, T)>,
}

impl<H, B, T> WrapHasher<H, B, T> {
    pub fn new(builder: B) -> Self {
        Self {
            builder: builder,
            phantom: PhantomData,
        }
    }
}

impl<H, B, T> Calculator for WrapHasher<H, B, T>
where
    H: Hasher,
    B: BuildHasher<Hasher=H>,
    T: Hash
{
    type Key = u64;
    type Value = T;

    fn calc_key(&self, v: &T) -> u64 {
        let mut hasher = self.builder.build_hasher();
        v.hash(&mut hasher);
        hasher.finish()
    }
}

pub type DefaultWrapHasher<T> = WrapHasher<DefaultHasher, RandomState, T>;

impl<T> std::default::Default for WrapHasher<DefaultHasher, RandomState, T> {
    fn default() -> Self {
        Self::new(RandomState::new())
    }
}
