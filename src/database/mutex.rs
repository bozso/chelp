use std::{
    sync::Mutex,
};
use crate::database::Database;

pub struct Mutexed<DB: Database> {
    db: Mutex<DB>,
}
