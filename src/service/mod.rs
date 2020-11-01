use std::{
    hash::BuildHasher,
    default::Default,
    collections::hash_map::RandomState,
};

use crate::{
    string,
    io,
    database::{
        default,
        Base,
        Database,
        indirect
    },
};


mod cresult;

pub type ID = u64;

pub use cresult::{CResult, CStatus};

#[derive(Debug)]
pub struct Service<B, S, F>
{
    pub string_service: string::Service<S>,
    pub file_service: io::Service<B, F>,
}

impl<B, S, F> Service<B, S, F>
where
    B: BuildHasher,
    S: Database<Entry = String>,
    F: Database<Entry = std::fs::File>,
{
    pub fn new(b: B, s: S, f: F) -> Self {
        Self {
            string_service: string::Service::new(s),
            file_service: io::Service::new(
                indirect::Indirect::new(b, f)
            ),
        }
    }
}

pub type DefaultService = Service<RandomState, 
    default::Default::<String>,
    Base::<std::fs::File>,
>;

impl Default for DefaultService {
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
