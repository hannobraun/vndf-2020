use hecs::{
    self,
    QueryBorrow,
};


pub struct Query<'r>(&'r mut hecs::World);

impl<'r> Query<'r> {
    pub fn new(inner: &'r mut hecs::World) -> Self {
        Self(inner)
    }

    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.0.query()
    }
}


pub type Entity = hecs::Entity;
