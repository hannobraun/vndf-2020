use crate::{
    cgs::{
        Handle,
        SecondaryStore,
    },
    game::{
        base::{
            Component,
            ComponentHandle,
        },
        crafts::Craft,
        explosions::Explosion,
        health::Health,
        missiles::Missile,
        physics::{
            Body,
            Position,
            Velocity,
        },
        ships::Ship,
    },
};


macro_rules! data {
    ($($name:ident, $ty:ident;)*) => {
        pub struct Data {
            $(pub $name: SecondaryStore<$ty>,)*
        }

        impl Data {
            pub fn new() -> Self {
                Self {
                    $($name: SecondaryStore::new(),)*
                }
            }

            pub fn update(&mut self, handle: Handle, component: Component) {
                match component {
                    $(
                        Component::$ty(value) => {
                            self.$name.insert(handle, value);
                        }
                    )*
                }
            }

            pub fn remove(&mut self, handle: ComponentHandle) {
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
    explosions, Explosion;
    healths,    Health;
    missiles,   Missile;
    positions,  Position;
    ships,      Ship;
    velocities, Velocity;
);
