use std::result::Result;

use crate::{
    service::Error
};


pub type ID = u64;

pub trait CService<Entry>: crate::database::Database<Entry = Entry> {
    fn must_get(&self, id: ID) -> Result<&Self::Entry, Error> {
        self.get(id).ok_or(Error::EntryNotFound(id))
    }
}
