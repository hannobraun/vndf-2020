use hecs::{
    self,
    DynamicBundle,
    NoSuchEntity,
    QueryBorrow,
};


pub struct Query<'r>(&'r mut hecs::World);

impl<'r> Query<'r> {
    pub fn new(inner: &'r mut hecs::World) -> Self {
        Self(inner)
    }

    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.0.spawn(components)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.0.despawn(entity)
    }

    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.0.query()
    }
}


pub type Entity = hecs::Entity;
