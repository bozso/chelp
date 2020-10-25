use std::{
    os::raw::c_char,
    result,
    error,
};

use once_cell::sync::Lazy;

pub mod database;
mod service;
pub mod string;

pub use database::{Database};

pub trait Error : error::Error + Into<service::ID> {}

pub type Result<T, E: Error> = result::Result<T, E>;


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
    return VERSION.clone();
}
static SERV: Lazy<service::DefaultService> = Lazy::new(
    || { service::DefaultService::default()}
);

#[no_mangle]
pub extern fn chelper_string(ptr: *mut c_char) -> service::CResult {
    SERV.string_service.put(ptr)
}
