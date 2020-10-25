use std::{
    os::raw::c_char,
    ffi::{CString, CStr},
};

use crate::{
    Result as LibResult,
    database::Database,
    service::{ID, CService},
};

pub enum Error {
    
}

type Result<T> = LibResult<T, Error>;

pub struct Service<DB> {
    db: DB,
}

impl<DB> Service<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db: db,
        }
    }
}

impl<DB: Database<Entry = String>> Service<DB> {
    pub fn put(&mut self, ptr: *mut c_char) -> Result<ID> {
        self.db.put(CString::from_raw(ptr).into_string()?)
    }
    
    fn concat(&mut self, one: ID, two: ID) -> Result<ID> {
        Ok(0 as ID)
    }
}

impl<DB: Database<Entry = String>> CService for Service<DB> {
    type Entry = String;
    
    fn get<'a>(&'a self, id: ID) -> Option<&'a Self::Entry> {
        self.db.get(id)
    }

}
