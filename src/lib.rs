use std::{
    os::raw::c_char,
    error,
    sync::{Mutex, MutexGuard, PoisonError},
    fmt::Debug,
    result::Result
};

use once_cell::sync::Lazy;

pub mod database;
pub mod service;
pub mod string;

pub use database::{Database};
pub use service::{CResult, ID};

pub trait Error : error::Error + Into<service::ID> {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VersionInfo {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    
}

const VERSION: VersionInfo = VersionInfo{
    major: 0,
    minor: 0,
    patch: 1,
};

#[no_mangle]
pub extern fn chelper_get_version() -> VersionInfo {
    VERSION.clone()
}

static SERV: Lazy<Mutex<service::DefaultService>> = Lazy::new(
    || { Mutex::new(service::DefaultService::default())}
);


use thiserror::Error as TError;

#[derive(TError, Debug)]
enum LError<T: Debug> {
    #[error("error while locking: {0}")]
    Lock(T),
    #[error("error while managing strings: {0}")]
    String(#[from] string::Error),
}

impl<T: Debug> Into<service::ID> for LError<T> {
    fn into(self) -> service::ID {
        3
    }
}

impl<T: Debug> Error for LError<T> {}

#[no_mangle]
pub extern fn chelper_string(ptr: *mut c_char) -> CResult {
    string_impl(ptr).into()
}

fn string_impl<'a>(ptr: *mut c_char) -> Result<ID, Box<dyn Error>> {
    SERV.lock().map_err(|e| LError::Lock(e))?
               .string_service.put(ptr)
    //SERV.lock()?.string_service.put(ptr)
}
