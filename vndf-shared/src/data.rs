use toadster::handle;

use crate::world::behavior::{
    crafts::{
        Craft,
        Fuel,
    },
    explosions::Explosion,
    health::Health,
    physics::{
        Body,
        Position,
        Velocity,
    },
    planets::Planet,
    players::Player,
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
        mod $module:ident($store_type:ident) {
            $($store_name:ident, $component_ty:ident;)*
        }
    ) => {
        pub mod $module {
            use serde::{
                Deserialize,
                Serialize,
            };
            use toadster::{
                handle::{
                    self,
                    Untyped,
                },
                store,
            };

            use super::{
                Remove,
                Update,
            };

            // Import all types passed to the macro into this module.
            $(
                use super::$component_ty;
            )*


            pub struct Components {
                $(pub $store_name: store::$store_type<$component_ty>,)*
            }

            impl Components {
                pub fn new() -> Self {
                    Self {
                        $($store_name: store::$store_type::new(),)*
                    }
                }
            }

            $(
                components!(@gen_update_remove, $store_type,
                    $store_name,
                    $component_ty,
                );
            )*


            #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
            pub enum Component {
                $(
                    $component_ty(
                        toadster::Handle<$component_ty>,
                        $component_ty,
                    ),
                )*
            }

            impl Component {
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
                impl From<(toadster::Handle<$component_ty>, $component_ty)>
                    for Component
                {
                    fn from(from:
                        (toadster::Handle<$component_ty>, $component_ty),
                    )
                        -> Self
                    {
                        Self::$component_ty(from.0, from.1)
                    }
                }
            )*


            #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
            pub enum Handle {
                $(
                    $component_ty(toadster::Handle<$component_ty>),
                )*
            }

            impl Handle {
                pub fn from_component(component: &Component) -> Self {
                    match component {
                        $(
                            Component::$component_ty(handle, _) =>
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
                impl From<(toadster::Handle<$component_ty>, $component_ty)>
                    for Handle
                {
                    fn from(
                        from: (toadster::Handle<$component_ty>, $component_ty),
                    )
                        -> Self
                    {
                        Self::$component_ty(from.0)
                    }
                }
            )*


            #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
            pub struct Diagnostics {
                $(pub $store_name: u64,)*
            }

            impl From<&Components> for Diagnostics {
                fn from(components: &Components) -> Self {
                    Self {
                        $($store_name: components.$store_name.len() as u64,)*
                    }
                }
            }
        }
    };

    (@gen_update_remove, Weak,
        $store_name:ident,
        $component_ty:ident,
    ) => {
        impl Update<$component_ty> for Components {
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

        impl Remove<$component_ty> for Components {
            fn remove(&mut self,
                handle: impl Into<handle::Weak<$component_ty>>,
            ) {
                self.$store_name.remove(handle);
            }
        }
    };

    (@gen_update_remove, Strong,
        $component_ty:ident,
        $store_name:ident,
    ) => {
        // Don't need to generate those for strong stores.
    };
}

components!(
    mod server(Strong) {
        bodies,     Body;
        crafts,     Craft;
        explosions, Explosion;
        fuels,      Fuel;
        healths,    Health;
        planets,    Planet;
        players,    Player;
        positions,  Position;
        ships,      Ship;
        velocities, Velocity;
    }
);

components!(
    mod client(Weak) {
        bodies,     Body;
        crafts,     Craft;
        explosions, Explosion;
        fuels,      Fuel;
        healths,    Health;
        planets,    Planet;
        positions,  Position;
        ships,      Ship;
        velocities, Velocity;
    }
);
