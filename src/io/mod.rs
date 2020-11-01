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
    service::ID,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

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
    pub fn open<P: HashPath>(&mut self, path: P) -> Result<ID> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Open})?
        )
    }

    pub fn create<P: HashPath>(&mut self, path: P) -> Result<ID> {
        Ok(self.indir.lazy_insert(
            &Creator{path: path, mode: Mode::Create})?
        )
    }
}

#[derive(Debug, Hash)]
pub enum Mode {
    Create,
    Open,
}

pub trait HashPath : AsRef<Path> + hash::Hash {}

#[derive(Debug, Hash)]
pub struct Creator<P> {
    path: P,
    mode: Mode,
}

impl<P: HashPath> crate::database::Creator for Creator<P> {
    type Entry = File;
    type Error = std::io::Error;
    
    fn create(&self) -> std::result::Result<Self::Entry, Self::Error> {
        match self.mode {
            Mode::Create => File::create(&self.path),
            Mode::Open => File::open(&self.path),
        }
    }
}
