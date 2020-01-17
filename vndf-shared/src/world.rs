use hecs::{
    self,
    Component,
    ComponentError,
    DynamicBundle,
    Entity,
    NoSuchEntity,
    QueryBorrow,
    Ref,
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

    pub fn spawn<'r>(&'r mut self, on_despawn: &'r mut dyn FnMut(Entity))
        -> Spawn<'r>
    {
        Spawn {
            world: &mut self.0,
            on_despawn,
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
}


pub struct Spawn<'r> {
    pub world:      &'r mut hecs::World,
    pub on_despawn: &'r mut dyn FnMut(Entity),
}

impl<'r> Spawn<'r> {
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        let entity = self.world.spawn(components);
        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)?;
        (self.on_despawn)(entity);
        Ok(())
    }
}
