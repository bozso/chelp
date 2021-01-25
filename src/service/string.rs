use std::{
    os::raw::c_char,
    ffi::CStr,
};

use crate::{
    database as db,
    service::{ID, Result},
};


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

impl<DB: db::AutoHash<String>> Service<DB> {
    pub fn put(&mut self, ptr: *const c_char) -> Result<ID> {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        Ok(self.db.insert_auto(cstr.to_string_lossy().into_owned()))
    }
    
    pub fn concat(&mut self, one: ID, two: ID) -> Result<ID> {
        let concatted = 
            self.db.must_get(&one)?.to_owned() + self.db.must_get(&two)?;
        
        Ok(self.db.insert_auto(concatted))
    }

    pub fn remove(&mut self, id: ID) -> Option<String> {
        self.db.remove(&id)
    }
}
