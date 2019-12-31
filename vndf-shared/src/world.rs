use hecs::{
    self,
    DynamicBundle,
    NoSuchEntity,
    Query,
    QueryBorrow,
};


pub struct World(hecs::World);

impl World {
    pub fn new() -> Self {
        Self(hecs::World::new())
    }

    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.0.spawn(components)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.0.despawn(entity)
    }

    pub fn query<Q: Query>(&self) -> QueryBorrow<Q> {
        self.0.query()
    }
}


pub type Entity = hecs::Entity;
