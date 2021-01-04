use std::{
    hash::BuildHasher,
    collections::hash_map::RandomState,
};

use crate::{
    service,
    database::{
        default,
        Base,
        Database,
        indirect
    },
};

#[derive(Debug)]
pub struct Service<B, S, F>
{
    pub string_service: service::string::Service<S>,
    pub file_service: service::io::Service<B, F>,
}

impl<B, S, F> Service<B, S, F>
where
    B: BuildHasher,
    S: Database<Entry = String>,
    F: Database<Entry = std::fs::File>,
{
    pub fn new(b: B, s: S, f: F) -> Self {
        Self {
            string_service: service::string::Service::new(s),
            file_service: service::io::Service::new(
                indirect::Indirect::new(b, f)
            ),
        }
    }
}

pub type Default = Service<RandomState, 
    default::Default::<String>,
    Base::<std::fs::File>,
>;

impl std::default::Default for Default {
    fn default() -> Self {
        Self::new(
            std::collections::hash_map::RandomState::new(),
            default::Default::<String>::default(),
            Base::<std::fs::File>::new(),
        )
    }
}

/*
impl<default::Default::<String>> Default for Service<default::Default::<String>> {
    fn default() -> Self {
        Self::new(default::Default::<String>::default())
    }
}
*/
