use std::{
    hash::BuildHasher,
    collections::hash_map::RandomState,
};

use crate::{
    service,
    database as db,
};

#[derive(Debug)]
pub struct Service<KC, DB, C, K>
{
    pub string_service: service::string::Service<DB>,
    pub file_service: service::io::Service<KC, DB, C>,
    tag: std::marker::PhantomData<K>,
}

impl<KC, DB, C, K> Service<KC, DB, C, K>
where
    DB: db::AutoHash<K>
{
    pub fn new(db: DB) -> Self {
        Self {
            string_service: service::string::Service::new(db),
            file_service: service::io::Service::new(db),
            tag: std::marker::PhantomData,
        }
    }
}
