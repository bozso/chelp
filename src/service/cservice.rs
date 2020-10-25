use std::{
    convert::From,
};

use crate::{
    Result, Error
};

#[repr(C)]
pub enum CStatus {
    Ok,
    Error
}

#[repr(C)]
pub struct CResult {
    id: ID,
    status: CStatus
} 

impl CResult {
    fn ok(id: ID) -> Self {
        Self {
            id: id,
            status: CResult::Ok,
        }
    }

    fn error(id: ID) -> Self {
        Self {
            id: id,
            status: CResult::Error,
        }
    }
}

impl<E: Error> From<Result<ID, E>> for CResult {
    fn from(r: Self::T) -> Self {
        match r {
            Ok(ok) => CResult::ok(ok),
            Err(err) => CResult::error(err.into()),
        }
    }
}

pub type ID = u64;

pub trait CService {
    fn get(&self, id: ID) -> Option<ID>;
    
    fn must_get(&self, id: ID) -> CResult {
        self.get_c().into()
    }
}
