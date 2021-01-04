pub trait System {
    type Entity;
    type ID;
    
    fn get(&self, &ID) -> &Entity;
}
