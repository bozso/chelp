use std::{
    result,
    error,
};

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

static serv: service::Services = service::Services::default();

//#[no_mangle]
//pub extern fn chelper_init() -> service::CResult {
    //Ok(0 as service::ID).into()
//}
