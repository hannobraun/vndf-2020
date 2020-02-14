use toadster::store;

use crate::game::{
    base::{
        Component,
        ComponentHandle,
    },
    crafts::{
        Craft,
        Fuel,
    },
    explosions::Explosion,
    health::Health,
    loot::Loot,
    missiles::{
        Missile,
        Target,
    },
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
    ships::Ship,
};


macro_rules! data {
    ($($name:ident, $ty:ident;)*) => {
        pub struct Data {
            $(pub $name: store::Weak<$ty>,)*
        }

        impl Data {
            pub fn new() -> Self {
                Self {
                    $($name: store::Weak::new(),)*
                }
            }

            pub fn update(&mut self, component: Component)
                -> bool
            {
                match component {
                    $(
                        Component::$ty(handle, value) => {
                            let previous = self.$name.insert(
                                handle,
                                value.clone(),
                            );
                            Some(value) != previous
                        }
                    )*
                }
            }

            pub fn remove(&mut self, handle: &ComponentHandle) {
                match handle {
                    $(
                        ComponentHandle::$ty(handle) => {
                            self.$name.remove(handle);
                        }
                    )*
                }
            }
        }
    };
}

data!(
    bodies,     Body;
    crafts,     Craft;
    directions, Direction;
    explosions, Explosion;
    fuels,      Fuel;
    healths,    Health;
    loots,      Loot;
    missiles,   Missile;
    positions,  Position;
    ships,      Ship;
    targets,    Target;
    velocities, Velocity;
);
