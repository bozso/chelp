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

pub type Result<T, K> = result::Result<T, Error<K>>;

const VERSION: VersionInfo = VersionInfo{
    major: 0,
    minor: 1,
    patch: 0,
};

#[no_mangle]
pub extern fn chelp_get_version() -> VersionInfo {
    VERSION.clone()
}

#[no_mangle]
pub extern fn chelp_new_service() -> *mut service::Default {
    Box::new(service::new_default())
}

/*
static SERV: Lazy<Mutex<service::Default>> = Lazy::new(
    || { Mutex::new(service::Default::default())}
);

fn lock<'a>() -> Result<MutexGuard<'a, service::Default>> {
    SERV.lock().map_err(|_| Error::LockFail)    
}

fn do_id<F: FnMut() -> Result<ID>>(mut func: F) -> CResult {
    func().into()
}

fn do_it<F: FnMut() -> Result<()>>(mut func: F) -> CResult {
    func().into()
}
*/

/**
 * Intern the string pointed to by `ptr` into the database. Returns the
 * id of the string.
 */
#[no_mangle]
pub extern fn chelp_string_intern(ptr: *const c_char) -> CResult {
    do_id(|| {
        lock()?.string_service.put(ptr).map_err(Error::Service)
    })
}

#[no_mangle]
pub extern fn chelp_string_concat(one: ID, two: ID) -> CResult {
    do_id(|| {
        lock()?.string_service.concat(one, two).map_err(Error::Service)
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
            .map_err(Error::Service)
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
                .map_err(|e| Error::Service(e.into()))
    })
}
