use hecs::{
    self,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    QueryBorrow,
    World,
};


pub struct Query<'r> {
    world: &'r mut World,
}

impl<'r> Query<'r> {
    pub fn new(world: &'r mut World) -> Self {
        Self {
            world,
        }
    }

    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.world.query()
    }
}


pub struct Spawn<'r> {
    pub world: &'r mut World,
}

impl<'r> Spawn<'r> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.spawn(components)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)
    }
}
