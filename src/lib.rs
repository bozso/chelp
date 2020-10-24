mod database;

#[repr(C)]
#[derive(Clone)]
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
