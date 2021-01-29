use std::{
    hash,
    path::Path,
    fs::File,
    convert::AsRef,
    marker::PhantomData,
};

use crate::{
    database::{self as db, Like},
    service::{ID},
};

#[derive(Debug, Hash)]
#[repr(C)]
pub enum Mode {
    Create,
    Open,
}

pub trait Service
{
    type Path: AsRef<Path>;
    type Error;

    fn open(&mut self, path: Self::Path) -> Result<ID, Self::Error>;
    fn create(&mut self, path: Self::Path) -> Result<ID, Self::Error>;
    fn close(&mut self, id: &ID) -> Result<(), Self::Error>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error<K: std::fmt::Debug> {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("failed to close file: {0}")]
    CloseFail(db::Error<K>),
}

#[derive(Debug, Hash)]
pub struct Creator<P, K> {
    path: P,
    mode: Mode,
    tag: PhantomData<K>,
}


impl<P, K> db::Creator for Creator<P, K>
where
    K: std::fmt::Debug + hash::Hash,
    P: AsRef<Path> + hash::Hash
{
    type Entry = File;
    type Error = Error<K>;
    
    fn create(&self) -> Result<Self::Entry, Self::Error> {
        match self.mode {
            Mode::Create => File::create(&self.path),
            Mode::Open => File::open(&self.path),
        }
    }
}

#[derive(Debug)]
pub struct Impl<KC, DB, P, C> {
    indir: db::id::Indirect<KC, File, DB, C>,
    tag: PhantomData<P>,
}

impl<KC, DB, P, C> db::Like for Impl<KC, DB, P, C> 
where
    KC: db::key::Calculator<Key = u64, Value = C>,
    C: db::Creator<Entry = File>,
    DB: db::Generic<u64, File>,
{
    type Key = u64;
    type Value = File;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.indir.get(key)
    }

    fn insert(&mut self, key: &Self::Key, val: Self::Value) {
        self.indir.insert(key, val)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.indir.remove(key)
    }

    fn contains(&self, key: &Self::Key) -> bool {
        self.indir.contains(key)
    }
}

impl<KC, DB, P, C> db::Generic<u64, File> for Impl<KC, DB, P, C> 
where
    KC: db::key::Calculator<Key = u64, Value = C>,
    C: db::Creator<Entry = File>,
    DB: db::Generic<u64, File>,
{}

impl<KC, DB, C, P> db::id::Generic<File> for Impl<KC, DB, P, C>
where
    KC: db::key::Calculator<Key = u64, Value = C>,
    C: db::Creator<Entry = File>,
    DB: db::Generic<u64, File>,
{}

impl<KC, DB, P, C> Impl<KC, DB, P, C> {
    pub fn new(key_calculator: KC, db: DB) -> Self 
    where
        KC: db::key::Calculator<Key = C>,
        DB: db::AutoHash<C> + db::id::Generic<File>
    {
        Self {
            indir: db::id::Indirect::new(key_calculator, db),
            tag: PhantomData,
        }
    }
}


impl<KC, DB, P> Service for Impl<KC, DB, P, Creator<P, u64>>
where
    KC: db::key::Calculator<Key = u64, Value = Creator<P, u64>>,
    DB: db::Generic<u64, File>,
    P: AsRef<Path> + hash::Hash
{
    type Path = P;
    type Error = <Creator<P, u64> as db::Creator>::Error;

    fn open(&mut self, path: P) -> Result<ID, Self::Error> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Open})?
        )
    }

    fn create(&mut self, path: P) -> Result<ID, Self::Error> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Create})?
        )
    }

    fn close(&mut self, id: &ID) -> Result<(), Self::Error> {
        match self.indir.remove(id).ok_or() {
            Some(_) => Ok(()),
            None => Error::CloseFail(db::Error::EntryNotFound(id)),
        }
    }
}
