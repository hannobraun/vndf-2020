use hecs::World;
use serde::{
    Deserialize,
    Serialize,
};


macro_rules! entity {
    ($($name:ident, $ty:ty;)*) => {
        use hecs::{
            ComponentError,
            NoSuchEntity,
        };


        #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
        pub struct Entity {
            pub id: Id,
            $(pub $name: Option<$ty>,)*
        }

        impl Entity {
            pub fn from_world(handle: hecs::Entity, world: &World) -> Self {
                let     id     = Id::from_handle(&handle);
                let mut entity = Self::new(id);

                $(
                    entity.$name = world.get::<$ty>(handle)
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
    pub fn from_handle(handle: &hecs::Entity) -> Self {
        Self(handle.to_bits())
    }
}


use crate::game::features::{
    crafts::components::Craft,
    explosions::components::Explosion,
    health::components::Health,
    missiles::components::Missile,
    physics::components::Body,
    ships::items::Ship,
};
entity!(
    body,      Body;
    craft,     Craft;
    explosion, Explosion;
    health,    Health;
    missile,   Missile;
    ship,      Ship;
);


#[cfg(test)]
mod tests {
    use hecs::World;
    use serde::{
        Deserialize,
        Serialize,
    };

    use super::Id;


    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    pub struct A(u64);

    impl A {
        pub fn new() -> Self {
            Self(0)
        }
    }


    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    pub struct B(u64);

    impl B {
        pub fn new() -> Self {
            Self(0)
        }
    }


    entity!(
        a, A;
        b, B;
    );


    #[test]
    fn it_should_create_an_entity_from_the_world() -> Result<(), Error> {
        let mut world  = World::new();
        let     entity = world.spawn((A::new(), B::new()));

        let entity = Entity::from_world(entity, &world);

        assert_eq!(entity.a, Some(A::new()));
        assert_eq!(entity.b, Some(B::new()));

        Ok(())
    }

    #[test]
    fn it_should_spawn_entities() -> Result<(), Error> {
        let mut world = World::new();

        let mut entity = Entity::new(Id(0));
        entity.a = Some(A::new());
        entity.b = Some(B::new());

        let id = entity.spawn(&mut world);

        assert_eq!(*world.get::<A>(id)?, entity.a.unwrap());
        assert_eq!(*world.get::<B>(id)?, entity.b.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_update_components() -> Result<(), Error> {
        let mut world = World::new();
        let     id    = world.spawn((A::new(), B::new()));

        let mut entity = Entity::new(Id(0));
        entity.a = Some(A(1));
        entity.b = Some(B(2));

        entity.update(id, &mut world)?;

        assert_eq!(*world.get::<A>(id)?, entity.a.unwrap());
        assert_eq!(*world.get::<B>(id)?, entity.b.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_add_components() -> Result<(), Error> {
        let mut world = World::new();
        let     id    = world.spawn(());

        let mut entity = Entity::new(Id(0));
        entity.a = Some(A::new());

        entity.update(id, &mut world)?;

        assert_eq!(*world.get::<A>(id)?, entity.a.unwrap());

        Ok(())
    }

    #[test]
    fn update_should_remove_components() -> Result<(), Error> {
        let mut world = World::new();
        let     id    = world.spawn((A::new(),));

        let entity = Entity::new(Id(0));
        entity.update(id, &mut world)?;

        assert!(world.get::<A>(id).is_err());

        Ok(())
    }


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
