use crate::{
    string,
    //database::Database,
};

mod cservice;

pub use cservice::{CService, ID, CResult, CStatus};

pub struct Services<DB> {
    string_service: string::Service<DB>,
}


impl<DB> Services<DB> {
    pub fn new() -> Self {
        Self {
            string_service: string::Service::new(),
        }
    }
}
