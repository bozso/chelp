use std::{
    sync::{Mutex, MutexGuard},
    os::raw::c_char,
    ffi::CStr,
    io::Write,
    result,
};

use once_cell::sync::Lazy;

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

static SERV: Lazy<Mutex<service::DefaultService>> = Lazy::new(
    || { Mutex::new(service::DefaultService::default())}
);

fn lock<'a>() -> Result<MutexGuard<'a, service::DefaultService>> {
    SERV.lock().map_err(|_| Error::LockFail)    
}

fn do_id<F: FnMut() -> Result<ID>>(mut func: F) -> CResult {
    func().into()
}

fn do_it<F: FnMut() -> Result<()>>(mut func: F) -> CResult {
    func().into()
}

#[no_mangle]
pub extern fn chelp_string_intern(ptr: *const c_char) -> CResult {
    do_id(|| {
        lock()?.string_service.put(ptr).map_err(Error::String)
    })
}

#[no_mangle]
pub extern fn chelp_string_concat(one: ID, two: ID) -> CResult {
    do_id(|| {
        lock()?.string_service.concat(one, two).map_err(Error::String)
    })
}

#[no_mangle]
pub extern fn chelp_string_remove(id: ID) -> CResult {
    do_it(|| {
        lock()?.string_service.remove(id);
        Ok(())
    })
}

#[no_mangle]
pub extern fn chelp_file_open(ptr: *const c_char) -> CResult {
    do_id(|| {
        let s = unsafe { CStr::from_ptr(ptr) };
        lock()?
            .file_service
            .open(s.to_string_lossy().into_owned())
            .map_err(Error::CIOError)
    })
}

#[no_mangle]
pub extern fn chelp_file_close(id: ID) -> CResult {
    do_it(|| {
        lock()?.file_service.must_remove(id)?;
        Ok(())
    })
}

#[no_mangle]
pub extern fn chelp_dump_db() -> CResult {
    do_it(|| {
        write!(std::io::stdout(), "Database: {:?}\n", lock()?)
            .map_err(|e| Error::IOError(e))
    })
}
