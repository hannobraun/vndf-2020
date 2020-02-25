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
    planet::Planet,
    ships::Ship,
};


macro_rules! components {
    (
        $name:ident($store:ident) {
            $($component_name:ident, $component_ty:ident;)*
        }
    ) => {
        pub struct $name {
            $(pub $component_name: store::$store<$component_ty>,)*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($component_name: store::$store::new(),)*
                }
            }

            pub fn update(&mut self, component: Component)
                -> bool
            {
                match component {
                    $(
                        Component::$component_ty(handle, value) => {
                            let previous = self.$component_name.insert(
                                &handle,
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
                        ComponentHandle::$component_ty(handle) => {
                            self.$component_name.remove(handle);
                        }
                    )*
                }
            }
        }
    };
}

components!(
    ClientData(Weak) {
        bodies,     Body;
        crafts,     Craft;
        directions, Direction;
        explosions, Explosion;
        fuels,      Fuel;
        healths,    Health;
        loots,      Loot;
        missiles,   Missile;
        planets,    Planet;
        positions,  Position;
        ships,      Ship;
        targets,    Target;
        velocities, Velocity;
    }
);