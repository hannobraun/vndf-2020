use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    handle::{
        self,
        Handle,
    },
    store,
};

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


/// Implemented for all generated collections of component stores
///
/// This trait doesn't do anything, but a trait is needed to make the generated
/// syntax in the macro work.
pub trait Components {}

/// Update component of a specific type from a collection of component stores
pub trait Update<T> {
    fn update(&mut self, handle: impl Into<handle::Weak<T>>, value: T) -> bool;
}

/// Remove component of a specific type from a collection of component stores
pub trait Remove<T> {
    fn remove(&mut self, handle: impl Into<handle::Weak<T>>);
}


macro_rules! components {
    (
        $components:ident($store_type:ident), $handle:ident, $component:ident {
            $($store_name:ident, $component_ty:ident;)*
        }
    ) => {
        pub struct $components {
            $(pub $store_name: store::$store_type<$component_ty>,)*
        }

        impl $components {
            pub fn new() -> Self {
                Self {
                    $($store_name: store::$store_type::new(),)*
                }
            }

            pub fn update(&mut self, component: Component)
                -> bool
            {
                match component {
                    $(
                        Component::$component_ty(handle, value) => {
                            let previous = self.$store_name.insert(
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
                            self.$store_name.remove(handle);
                        }
                    )*
                }
            }
        }

        impl Components for $components {}

        $(
            impl Update<$component_ty> for $components {
                fn update(&mut self,
                    handle: impl Into<handle::Weak<$component_ty>>,
                    value:  $component_ty,
                )
                    -> bool
                {
                    let previous = self.$store_name.insert(
                        handle,
                        value.clone(),
                    );
                    Some(value) != previous
                }
            }

            // This is currently generated for all store types, but will only
            // compile for weak stores. Once the macro invocation is extended to
            // generate strong stores, we'll have to be a bit smarter here.
            impl Remove<$component_ty> for $components {
                fn remove(&mut self,
                    handle: impl Into<handle::Weak<$component_ty>>,
                ) {
                    self.$store_name.remove(handle);
                }
            }
        )*


        #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
        pub enum $handle {
            $(
                $component_ty(Handle<$component_ty>),
            )*
        }

        impl $handle {
            pub fn remove<T>(&self, components: &mut T)
                where T: Components $(+ Remove<$component_ty>)*
            {
                match self {
                    $(
                        Self::$component_ty(handle) => {
                            <T as Remove<$component_ty>>::remove(
                                components,
                                handle,
                            );
                        }
                    )*
                }
            }
        }


        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub enum $component {
            $(
                $component_ty(Handle<$component_ty>, $component_ty),
            )*
        }

        impl $component {
            pub fn update<T>(self, components: &mut T)
                where T: Components $(+ Update<$component_ty>)*
            {
                match self {
                    $(
                        Self::$component_ty(handle, value) => {
                            <T as Update<$component_ty>>::update(
                                components,
                                handle,
                                value,
                            );
                        }
                    )*
                }
            }
        }
    };
}

components!(
    ClientData(Weak), ClientHandle, ClientComponent {
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
