use std::{
    convert::From,
    result::Result,
};

use crate::{
    Error,
    service::ID,
};

pub const OK: ID = 0;

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
            id,
            status: CStatus::Ok,
        }
    }

    fn error(id: ID) -> Self {
        Self {
            id,
            status: CStatus::Error,
        }
    }
}

impl<E: Error> From<Result<ID, E>> for CResult {
    fn from(r: Result<ID, E>) -> Self {
        match r {
            Ok(ok) => CResult::ok(ok),
            Err(err) => CResult::error(err.into()),
        }
    }
}

impl<E: Error> From<Result<(), E>> for CResult {
    fn from(r: Result<(), E>) -> Self {
        match r {
            Ok(()) => CResult::ok(OK),
            Err(err) => CResult::error(err.into()),
        }
    }
}
