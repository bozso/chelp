//use std::{
    //os::raw::c_char,
    //ffi::{CString, CStr},
//};

use crate::{
    database::Database,
    service::{CService, ID},
};

pub struct Service<DB> {
    db: DB,
}

impl<DB: Database<Entry = String>> Service<DB> {
    fn concat(&mut self, one: ID, two: ID) -> Result<ID> {
        
    }
}

impl<DB: Database<Entry = String>> CService for Service<DB> {
    fn get(&self, id: ID) -> Option<ID> {
        self.db.get(id)
    }
}
