use std::{
    hash,
    path::Path,
    fs::File,
    convert::AsRef,
    collections::hash_map::RandomState,
};

use crate::{
    database::{
        Database,
        indirect::Indirect,
    },
    service::{ID, Result, Error},
};


#[derive(Debug)]
pub struct Service<B, DB> {
    indir: Indirect<File, B, DB>,
}

impl<B, DB> Service<B, DB> {
    pub fn new(indir: Indirect<File, B, DB>) -> Self {
        Self {
            indir,
        }
    }
}

impl<DB: Database<Entry = File>> Service<RandomState, DB> {
    pub fn with_db(db: DB) -> Self {
        Self::new(Indirect::with_db(db))
    }
}

impl<B, DB> Service<B, DB>
where
    B: hash::BuildHasher,
    DB: Database<Entry = std::fs::File>
{
    pub fn open<P>(&mut self, path: P) -> Result<ID>
    where
        P: AsRef<Path> + hash::Hash
    {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Open})?
        )
    }

    pub fn create<P>(&mut self, path: P) -> Result<ID>
    where
        P: AsRef<Path> + hash::Hash
    {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Create})?
        )
    }

    pub fn must_get(&self, id: ID) -> Result<&File> {
        self.indir.must_get(id).map_err(Error::DBError)
    }

    pub fn must_remove(&mut self, id: ID) -> Result<File> {
        self.indir.must_remove(id).map_err(Error::DBError)
    }
}

#[derive(Debug, Hash)]
pub enum Mode {
    Create,
    Open,
}

#[derive(Debug, Hash)]
pub struct Creator<P> {
    path: P,
    mode: Mode,
}

impl<P> crate::database::Creator for Creator<P>
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
