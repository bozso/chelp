use std::{
    marker::PhantomData,
    collections::HashMap,
    hash::{Hasher, Hash}
};

use crate::database::{Database, HashMaker};

pub struct Default<T: Hash, H: Hasher, M: HashMaker<H>> {
    maker: M,
    db: HashMap<u64, T>,
    phantom: PhantomData<H>,
}



impl<T: Hash, H: Hasher, M: HashMaker<H>> Default<T, H, M> {
    fn calc_id(&self, entry: &T) -> u64 {
        let mut hasher = self.maker.make();
        entry.hash(&mut hasher);
        hasher.finish()
    }
}


impl<T: Hash, H: Hasher, M: HashMaker<H>> Database for Default<T, H, M> {
    type Entry = T;
    
    fn put(&mut self, entry: Self::Entry) {
        self.db.insert(self.calc_id(&entry), entry);
    }
    
    fn get(&self, id: u64) -> Option<&Self::Entry> {
        self.db.get(&id)
    }
    
}
