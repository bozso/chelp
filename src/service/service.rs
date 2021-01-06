use std::{
    hash::BuildHasher,
    collections::hash_map::RandomState,
};

use crate::{
    service,
    database as db,
};

#[derive(Debug)]
pub struct Service<DB>
{
    pub string_service: service::string::Service<DB>,
    pub file_service: service::io::Service<DB>,
}

impl<DB: db::AutoHash> Service<DB>
{
    pub fn new(db: DB) -> Self {
        Self {
            string_service: service::string::Service::new(db),
            file_service: service::io::Service::new(db),
        }
    }
}
