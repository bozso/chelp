use std::{
    os::raw::c_char,
    ffi::CStr,
};

use crate::{
    database as db,
    service::{ID, Result},
};


pub trait Service {
    type Key;
    type Error: std::error::Error;

    fn put(&mut self, ptr: *const c_char) -> Self::Key;
    fn concat(&mut self, one: Self::Key, two: Self::Key) -> Result<Self::Key, Self::Error>;
    fn delete(&mut self, id: Self::Key);
}

pub struct Impl<DB> {
    db: DB,
}



impl<DB> Impl<DB> {
    pub fn new(db: DB) -> Self {
        Self {
            db,
        }
    }
}

impl<DB: db::AutoHash<String>> Impl<DB> {
    fn remove(&mut self, id: ID) -> Option<String> {
        self.db.remove(&id)
    }
}

impl<DB: db::AutoHash<String>> Service for Impl<DB> {
    type Key = ID;
    type Error = db::Error<Self::Key>;

    fn put(&mut self, ptr: *const c_char) -> Self::Key {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        self.db.insert_auto(cstr.to_string_lossy().into_owned())
    }
    
    fn concat(&mut self, one: ID, two: ID) -> Result<Self::Key, Self::Error> {
        let concatted = 
            self.db.must_get(&one)?.to_owned() + self.db.must_get(&two)?;
        
        Ok(self.db.insert_auto(concatted))
    }

    fn delete(&mut self, key: Self::Key) {
        self.remove(key);
    }
}
