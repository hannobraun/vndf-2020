use hecs::{
    self,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    Query,
    QueryBorrow,
};


pub struct World {
    inner: hecs::World,
}

impl World {
    pub fn new() -> Self {
        Self {
            inner: hecs::World::new(),
        }
    }

    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.inner.spawn(components)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.inner.despawn(entity)
    }

    pub fn query<Q: Query>(&self) -> QueryBorrow<Q> {
        self.inner.query()
    }
}
