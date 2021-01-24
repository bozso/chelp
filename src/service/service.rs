use crate::{
    service,
    database as db,
};

#[derive(Debug)]
pub struct Service<SS, IOS>
{
    pub string_service: SS,
    pub io_service: IOS,
}

impl<SS, IOS> Service<SS, IOS> {
    pub fn new(ss: SS, ios: IOS) -> Self {
        Self {
            string_service: ss,
            file_service: ios,
        }
    }
}
