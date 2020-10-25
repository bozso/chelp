use std::{
    ffi::CStr,
    
};

pub trait CService {
    type CPtr;
    type RustRef;
    
    fn get(&self, id: u64) -> Option<RustRef>;
    fn get_c(&self, id: u64) -> Option<CPtr>;
}
