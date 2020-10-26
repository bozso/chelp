use std::result::Result;

use crate::{
    service::Error
};


pub type ID = u64;

pub trait CService {
    type Entry;

    fn get(&self, id: ID) -> Option<&Self::Entry>;
    
    fn must_get(&self, id: ID) -> Result<&Self::Entry, Error> {
        self.get(id).ok_or(Error::EntryNotFound(id))
    }
}
