use std::{
    result,
    error,
};

mod database;
mod service;
mod string;

pub use database::{Database, Default};

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

static serv: service::Services<database::Default> = service::Services::new();

#[no_mangle]
pub extern fn chelper_init() -> service::CResult {
    
}
