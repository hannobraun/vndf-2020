use hecs::{
    self,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    QueryBorrow,
};


pub struct DeSpawned {
    pub spawned:   Vec<Entity>,
    pub despawned: Vec<Entity>,
}

impl DeSpawned {
    pub fn new() -> Self {
        Self {
            spawned:   Vec::new(),
            despawned: Vec::new(),
        }
    }
}


pub struct Query<'r> {
    pub world: &'r mut hecs::World,
}

impl<'r> Query<'r> {
    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.world.query()
    }
}


pub struct Spawn<'r> {
    pub world:      &'r mut hecs::World,
    pub de_spawned: &'r mut DeSpawned,
}

impl<'r> Spawn<'r> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        let entity = self.world.spawn(components);
        self.de_spawned.spawned.push(entity);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)?;
        self.de_spawned.despawned.push(entity);
        Ok(())
    }
}
