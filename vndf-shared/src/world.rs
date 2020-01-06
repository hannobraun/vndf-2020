use hecs::{
    self,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    QueryBorrow,
};


pub struct Query<'r> {
    pub world: &'r mut hecs::World,
}

impl<'r> Query<'r> {
    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.world.query()
    }
}


pub struct Spawn<'r> {
    pub world:     &'r mut hecs::World,
    pub spawned:   &'r mut Vec<Entity>,
    pub despawned: &'r mut Vec<Entity>,
}

impl<'r> Spawn<'r> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        let entity = self.world.spawn(components);
        self.spawned.push(entity);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)?;
        self.despawned.push(entity);
        Ok(())
    }
}
