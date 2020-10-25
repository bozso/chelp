use std::result::Result;

use crate::{
    service::Error
};


pub type ID = u64;

pub trait CService {
    type Entry;

    fn get<'a>(&'a self, id: ID) -> Option<&'a Self::Entry>;
    
    fn must_get<'a> (&'a self, id: ID) -> Result<&'a Self::Entry, Error> {
        self.get(id).ok_or(Error::EntryNotFound(id))
    }
}
