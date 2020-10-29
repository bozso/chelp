use std::{
    path::Path,
    convert::AsRef,
};

use crate::{
    database as db,
    service::{ID, CService},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Service<DB> {
    db: DB,
}

impl<DB> Service<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db,
        }
    }
}

enum ReadWrite {
    Read(Box<dyn std::io::Read>),
    Write(Box<dyn std::io::Write>),
}

impl<DB: db::Database<Entry = std::fs::File>> Service<DB> {
    fn lazy_put<F>(&mut self, f: F, id: ID) -> Result<()>
    where
        F: Fn() -> std::io::Result<std::fs::File>
    {
        
    }
    
    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<ID> {
        Ok(self.db.insert(std::fs::File::open(path)?))
    }

    pub fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<ID> {
        Ok(self.db.insert(std::fs::File::create(path)?))
    }
}

type DBEntry = std::fs::File;

impl<DB: db::Database<Entry = DBEntry>> db::Database for Service<DB> {
    type Entry = DBEntry;
    
    fn get(&self, id: ID) -> Option<&Self::Entry> {
        self.db.get(id)
    }
    
    fn insert(&mut self, entry: Self::Entry) -> ID {
        self.db.insert(entry)
    }
    
    fn remove(&mut self, id: ID) {
        self.db.remove(id)
    }
}

impl<DB: db::Database<Entry = DBEntry>> CService<DBEntry> for Service<DB> {}

