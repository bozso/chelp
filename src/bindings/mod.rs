use std::{
    sync::{Mutex, MutexGuard},
    os::raw::c_char,
    result,
};

use crate::{
    service,
    service::{ID, CResult},
};

mod error;
pub use error::Error;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VersionInfo {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    
}

pub type Result<T> = result::Result<T, Error>;

const VERSION: VersionInfo = VersionInfo{
    major: 0,
    minor: 1,
    patch: 0,
};

#[no_mangle]
pub extern fn chelp_get_version() -> VersionInfo {
    VERSION.clone()
}

use once_cell::sync::Lazy;

static SERV: Lazy<Mutex<service::DefaultService>> = Lazy::new(
    || { Mutex::new(service::DefaultService::default())}
);


fn lock<'a>() -> Result<MutexGuard<'a, service::DefaultService>> {
    SERV.lock().map_err(|_| Error::LockFail)    
}

#[no_mangle]
pub extern fn chelp_intern_string(ptr: *mut c_char) -> CResult {
    string_impl(ptr).into()
}

fn string_impl(ptr: *mut c_char) -> Result<ID> {
    SERV.lock().map_err(|_| Error::LockFail)?
               .string_service.put(ptr)
               .map_err(Error::String)
}

#[no_mangle]
pub extern fn chelp_concat_strings(one: ID, two: ID) -> CResult {
    concat_impl(one, two).into()
}

fn concat_impl(one: ID, two: ID) -> Result<ID> {
    lock()?.string_service.concat(one, two)
           .map_err(Error::String)
}

