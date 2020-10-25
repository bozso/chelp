use crate::{
    string,
    database::Database,
};

mod cservice;

pub use cservice::CService;

pub struct Services<SS: string::Service> {
    string_service: SS,
}
