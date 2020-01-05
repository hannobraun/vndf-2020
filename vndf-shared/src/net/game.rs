use hecs::{
    ComponentError,
    NoSuchEntity,
    World,
};
use serde::{
    Deserialize,
    Serialize,
};


macro_rules! entity {
    ($($name:ident, $ty:ident;)*) => {
        use crate::game::components::{
            $($ty,)*
        };

        #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
        pub struct Entity {
            pub id: Id,
            $(pub $name: Option<$ty>,)*
        }

        impl Entity {
            pub fn from_world(hecs_id: hecs::Entity, world: &World) -> Self {
                let     id     = Id::from_hecs_entity(&hecs_id);
                let mut entity = Self::new(id);

                $(
                    entity.$name = world.get::<$ty>(hecs_id)
                        .ok()
                        .map(|component_ref| (*component_ref).clone());
                )*

                entity
            }

            pub fn new(id: Id) -> Self {
                Entity {
                    id,
                    $($name: None,)*
                }
            }

            pub fn spawn(&self, world: &mut World) -> hecs::Entity {
                let entity = world.spawn(());

                $(
                    if let Some(component) = self.$name {
                        world.insert_one(entity, component)
                            .expect("Entity we just created doesn't exist");
                    }
                )*

                entity
            }

            pub fn update(&self, entity: hecs::Entity, world: &mut World)
                -> Result<(), NoSuchEntity>
            {
                $(
                    match self.$name {
                        Some(component) => {
                            world.insert_one(entity, component)?;
                        }
                        None => {
                            match world.remove_one::<$ty>(entity) {
                                Err(ComponentError::NoSuchEntity) => {
                                    return Err(NoSuchEntity);
                                }
                                Err(ComponentError::MissingComponent(_)) => {
                                    // We just want the entity to have no such
                                    // component. If it wasn't there in the
                                    // first place, fine by us!
                                }
                                Ok(_) => (),
                            }
                        }
                    }
                )*

                Ok(())
            }
        }
    };
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize, Hash)]
pub struct Id(pub u64);

impl Id {
    pub fn from_hecs_entity(entity: &hecs::Entity) -> Self {
        Self(entity.to_bits())
    }
}


entity!(
    body,      Body;
    engine,    Engine;
    explosion, Explosion;
    missile,   Missile;
    ship,      Ship;
);


#[cfg(test)]
mod tests {
    use hecs::World;

    use crate::{
        game::components::{
            Body,
            Ship,
        },
        math::{
            prelude::*,
            Rad,
        },
    };

    use super::{
        Entity,
        Id,
    };


    #[test]
    fn it_should_create_an_entity_from_the_world() -> Result {
        let mut world  = World::new();
        let     entity = world.spawn((Body::new(), Ship::new()));

        let entity = Entity::from_world(entity, &world);

        assert_eq!(entity.body, Some(Body::new()));
        assert_eq!(entity.ship, Some(Ship::new()));

        Ok(())
    }

    #[test]
    fn it_should_spawn_entities() -> Result {
        let mut world = World::new();

        let mut entity = Entity::new(Id(0));
        entity.body = Some(Body::new());
        entity.ship = Some(Ship::new());

        let id = entity.spawn(&mut world);

        assert_eq!(*world.get::<Body>(id)?, entity.body.unwrap());
        assert_eq!(*world.get::<Ship>(id)?, entity.ship.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_update_components() -> Result {
        let mut world = World::new();
        let     id    = world.spawn((Body::new(), Ship::new()));

        let mut entity = Entity::new(Id(0));
        entity.body = Some(Body { dir: Rad::full_turn(), .. Body::new()});
        entity.ship = Some(Ship { missiles: u64::max_value(), .. Ship::new()});

        entity.update(id, &mut world)?;

        assert_eq!(*world.get::<Body>(id)?, entity.body.unwrap());
        assert_eq!(*world.get::<Ship>(id)?, entity.ship.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_add_components() -> Result {
        let mut world = World::new();
        let     id    = world.spawn(());

        let mut entity = Entity::new(Id(0));
        entity.body = Some(Body::new());

        entity.update(id, &mut world)?;

        assert_eq!(*world.get::<Body>(id)?, entity.body.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_remove_components() -> Result {
        let mut world = World::new();
        let     id    = world.spawn((Body::new(),));

        let entity = Entity::new(Id(0));
        entity.update(id, &mut world)?;

        assert!(world.get::<Body>(id).is_err());

        Ok(())
    }


    type Result<T = ()> = std::result::Result<T, Error>;


    #[derive(Debug)]
    enum Error {
        ComponentError(hecs::ComponentError),
        NoSuchEntity(hecs::NoSuchEntity),
    }

    impl From<hecs::ComponentError> for Error {
        fn from(err: hecs::ComponentError) -> Self {
            Self::ComponentError(err)
        }
    }

    impl From<hecs::NoSuchEntity> for Error {
        fn from(err: hecs::NoSuchEntity) -> Self {
            Self::NoSuchEntity(err)
        }
    }
}
