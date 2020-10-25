mod base;
mod default;

pub use base::Base;
pub use default::Default;

pub trait Database {
    type Entry;
    
    fn put(&mut self, entry: Self::Entry);
    fn get(&self, id: u64) -> Option<&Self::Entry>;
}
