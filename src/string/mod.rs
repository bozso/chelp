use std::{
    os::raw::c_char,
    ffi::{CStr, IntoStringError},
    result,
};

use crate::{
    Error as LibError,
    database::Database,
    service::{ID, CService},
    service,
};

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("error while converting from pointer: {0}")]
    ConversionError(#[from] IntoStringError),
    #[error("error in C-service: {0}")]
    ServiceError(#[from] service::Error),
}

impl Into<ID> for Error {
    fn into(self) -> ID {
        match self {
            Self::ConversionError(_) => 2,
            Self::ServiceError(e) => e.into(),
        }
    }
}

impl LibError for Error {}

type Result<T> = result::Result<T, Error>;

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

impl<DB: Database<Entry = String>> Service<DB> {
    pub fn put(&mut self, ptr: *const c_char) -> Result<ID> {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        Ok(self.db.insert(cstr.to_string_lossy().into_owned()))
    }
    
    pub fn concat(&mut self, one: ID, two: ID) -> Result<ID> {
        let concatted = 
            self.must_get(one)?.to_owned() + self.must_get(two)?;
        
        Ok(self.db.insert(concatted))
    }
}

impl<DB: Database<Entry = String>> Database for Service<DB> {
    type Entry = String;
    
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

impl<DB: Database<Entry = String>> CService<String> for Service<DB> {}
