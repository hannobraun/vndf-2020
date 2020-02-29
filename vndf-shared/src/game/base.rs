pub mod events;
pub mod feature;


pub use self::{
    events::*,
    feature::*,
};


use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    handle::{
        self,
        Untyped,
    },
};

use crate::{
    data::{
        ClientComponent,
        ClientHandle,
    },
    game::{
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
    },
};


macro_rules! components {
    ($($component:ident,)*) => {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub enum Component {
            $($component(Handle<$component>, $component),)*
        }

        #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
        pub enum ComponentHandle {
            $($component(Handle<$component>),)*
        }

        impl ComponentHandle {
            pub fn from_component(component: &Component) -> Self {
                match component {
                    $(
                        Component::$component(handle, _) =>
                            ComponentHandle::$component(handle.clone()),
                    )*
                }
            }

            pub fn is_strong(&self) -> bool {
                match self {
                    $(
                        Self::$component(handle) => handle.is_strong(),
                    )*
                }
            }

            pub fn as_weak(&self) -> Self {
                match self {
                    $(
                        Self::$component(handle) =>
                            Self::$component(handle.as_weak()),
                    )*
                }
            }

            pub fn into_weak(self) -> Self {
                match self {
                    $(
                        Self::$component(handle) =>
                            Self::$component(handle.into_weak()),
                    )*
                }
            }

            pub fn into_strong_untyped(self) -> handle::Strong<Untyped> {
                match self {
                    $(
                        Self::$component(handle) =>
                            handle.strong().into_untyped(),
                    )*
                }
            }

            pub fn into_weak_untyped(self) -> handle::Weak<Untyped> {
                match self {
                    $(
                        Self::$component(handle) =>
                            handle.weak().into_untyped(),
                    )*
                }
            }
        }

        impl From<Component> for ClientComponent {
            fn from(from: Component) -> Self {
                match from {
                    $(
                        Component::$component(handle, value) =>
                            Self::$component(handle, value),
                    )*
                }
            }
        }

        impl From<ComponentHandle> for ClientHandle {
            fn from(from: ComponentHandle) -> Self {
                match from {
                    $(
                        ComponentHandle::$component(handle) =>
                            Self::$component(handle),
                    )*
                }
            }
        }
    };
}

components!(
    Body,
    Craft,
    Direction,
    Explosion,
    Fuel,
    Health,
    Loot,
    Missile,
    Planet,
    Position,
    Ship,
    Target,
    Velocity,
);
