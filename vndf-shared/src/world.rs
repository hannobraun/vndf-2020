use hecs::{
    self,
    Component,
    ComponentError,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    QueryBorrow,
    Ref,
    RefMut,
};


pub struct World(hecs::World);

impl World {
    pub fn new() -> Self {
        Self(hecs::World::new())
    }

    pub fn query(&mut self) -> Query {
        Query {
            world: &mut self.0,
        }
    }

    pub fn spawn<'r>(&'r mut self, despawned: &'r mut Vec<Entity>)
        -> Spawn<'r>
    {
        Spawn {
            world: &mut self.0,
            despawned,
        }
    }

    pub fn inner(&self) -> &hecs::World {
        &self.0
    }
}


pub struct Query<'r> {
    pub world: &'r mut hecs::World,
}

impl<'r> Query<'r> {
    pub fn query<Q: hecs::Query>(&self) -> QueryBorrow<Q> {
        self.world.query()
    }

    pub fn get<T: Component>(&self, entity: Entity)
        -> Result<Ref<T>, ComponentError>
    {
        self.world.get(entity)
    }

    pub fn get_mut<T: Component>(&self, entity: Entity)
        -> Result<RefMut<T>, ComponentError>
    {
        self.world.get_mut(entity)
    }
}


pub struct Spawn<'r> {
    pub world:     &'r mut hecs::World,
    pub despawned: &'r mut Vec<Entity>,
}

impl<'r> Spawn<'r> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        let entity = self.world.spawn(components);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)?;
        self.despawned.push(entity);
        Ok(())
    }
}
