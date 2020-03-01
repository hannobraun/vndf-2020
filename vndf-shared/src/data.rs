use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    handle::{
        self,
        Handle,
        Untyped,
    },
    store,
};

use crate::game::{
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
        }

        $(
            components!(@gen_update_remove, $store_type,
                $component_ty,
                $components,
                $store_name,
            );
        )*


        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub enum $component {
            $(
                $component_ty(Handle<$component_ty>, $component_ty),
            )*
        }

        impl $component {
            pub fn update<T>(self, components: &mut T) -> bool
                where T: $(Update<$component_ty> +)*
            {
                match self {
                    $(
                        Self::$component_ty(handle, value) => {
                            <T as Update<$component_ty>>::update(
                                components,
                                handle,
                                value,
                            )
                        }
                    )*
                }
            }
        }

        $(
            impl From<(Handle<$component_ty>, $component_ty)> for $component {
                fn from(from: (Handle<$component_ty>, $component_ty)) -> Self {
                    Self::$component_ty(from.0, from.1)
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
            pub fn from_component(component: &$component) -> Self {
                match component {
                    $(
                        $component::$component_ty(handle, _) =>
                            Self::$component_ty(handle.clone()),
                    )*
                }
            }

            pub fn as_weak(&self) -> Self {
                match self {
                    $(
                        Self::$component_ty(handle) =>
                            Self::$component_ty(handle.as_weak()),
                    )*
                }
            }

            pub fn into_strong_untyped(self) -> handle::Strong<Untyped> {
                match self {
                    $(
                        Self::$component_ty(handle) =>
                            handle.strong().into_untyped(),
                    )*
                }
            }

            pub fn into_weak_untyped(self) -> handle::Weak<Untyped> {
                match self {
                    $(
                        Self::$component_ty(handle) =>
                            handle.weak().into_untyped(),
                    )*
                }
            }

            pub fn remove<T>(&self, components: &mut T)
                where T: $(Remove<$component_ty> +)*
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

        $(
            impl From<(Handle<$component_ty>, $component_ty)> for $handle {
                fn from(from: (Handle<$component_ty>, $component_ty)) -> Self {
                    Self::$component_ty(from.0)
                }
            }
        )*
    };

    (@gen_update_remove, Weak,
        $component_ty:ident,
        $components:ident,
        $store_name:ident,
    ) => {
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

        impl Remove<$component_ty> for $components {
            fn remove(&mut self,
                handle: impl Into<handle::Weak<$component_ty>>,
            ) {
                self.$store_name.remove(handle);
            }
        }
    };

    (@gen_update_remove, Strong,
        $component_ty:ident,
        $components:ident,
        $store_name:ident,
    ) => {
        // Don't need to generate those for strong stores.
    };
}

components!(
    ServerData(Strong), ServerHandle, ServerComponent {
        bodies, Body;
    }
);

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
