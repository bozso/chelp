use std::{
    hash,
    path::Path,
    fs::File,
    convert::AsRef,
};

use crate::{
    database as db,
    service::{ID, Result},
};

#[derive(Debug, Hash)]
#[repr(C)]
pub enum Mode {
    Create,
    Open,
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
pub struct Service<KC, DB, C> {
    indir: db::id::Indirect<KC, File, DB, C>,
}

impl<KC, DB, C> Service<KC, DB, C> {
    pub fn new(db: DB) -> Self 
    where
        DB: db::id::Generic<File>
    {
        Self {
            indir: db::id::Indirect::new(db),
        }
    }
}

impl<KC, DB, C> Service<KC, DB, C>
where
    DB: db::id::Generic<std::fs::File>
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
}


