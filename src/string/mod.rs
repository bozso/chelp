use std::{
    os::raw::c_char,
    ffi::{CString, IntoStringError},
    result,
};

use crate::{
    Error as LibError,
    database::Database,
    service::ID,
};

use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("error while converting from pointer: {0}")]
    ConversionError(#[from] IntoStringError),
}

impl Into<ID> for Error {
    fn into(self) -> ID {
        match self {
            Self::ConversionError(_) => 2,
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
    pub fn put(&mut self, ptr: *mut c_char) -> Result<ID> {
        let cstr = unsafe { CString::from_raw(ptr) };
        Ok(self.db.put(cstr.into_string()?))
    }
}
/*
impl<DB: Database<Entry = String, Error = SError>> CService for Service<DB> {
    fn get<'a>(&'a self, id: ID) -> Option<&'a Self::Entry> {
        self.db.get(id)
    }

}
*/
