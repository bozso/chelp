use std::{
    hash,
    path::Path,
    fs::File,
    convert::AsRef,
    marker::PhantomData,
};

use crate::{
    database::{self as db, Like},
    service::{ID, Result},
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

    fn open(&mut self, path: Self::Path) -> Result<ID>;
    fn create(&mut self, path: Self::Path) -> Result<ID>;
    fn close(&mut self, id: &ID) -> Result<()>;
}

#[derive(Debug, Hash)]
pub struct Creator<P> {
    path: P,
    mode: Mode,
}

impl<P> db::Creator for Creator<P>
where
    P: AsRef<Path> + hash::Hash
{
    type Entry = File;
    type Error = std::io::Error;
    
    fn create(&self) -> std::result::Result<Self::Entry, Self::Error> {
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


impl<KC, DB, P> Service for Impl<KC, DB, P, Creator<P>>
where
    KC: db::key::Calculator<Key = u64, Value = Creator<P>>,
    DB: db::Generic<u64, File>,
    P: AsRef<Path> + hash::Hash
{
    type Path = P;

    fn open(&mut self, path: P) -> Result<ID> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Open})?
        )
    }

    fn create(&mut self, path: P) -> Result<ID> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Create})?
        )
    }

    fn close(&mut self, id: &ID) -> Result<()> {
        self.indir.remove(id);
        Ok(())
    }
}
