
use crate::{
    service::{Result, Error}
};


pub type ID = u64;

pub trait CService {
    type Entry;
    fn get(&self, id: ID) -> Option<Self::Entry>;
    
    fn must_get(&self, id: ID) -> Result<Self::Entry> {
        self.get(id).ok_or(Error::EntryNotFound{id: id})
    }
}
