use std::{
    collections::VecDeque,
    net::SocketAddr,
};

use hecs::Entity;

use crate::{
    game::entities::{
        Explosion,
        Missile,
    },
    input,
};


pub struct Events(VecDeque<Event>);

impl Events {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self) -> Push {
        Push(&mut self.0)
    }

    pub fn next(&mut self) -> Option<Event> {
        self.0.pop_front()
    }
}


macro_rules! events {
    (
        $(
            $event:ident, $event_lower:ident {
                $($arg_name:ident: $arg_type:ty,)*
            }
        )*
    ) => {
        pub struct Push<'r>(&'r mut VecDeque<Event>);

        impl Push<'_> {
            $(
                pub fn $event_lower(&mut self, $($arg_name: $arg_type,)*) {
                    self.0.push_back(Event::$event { $($arg_name,)* });
                }
            )*
        }


        pub enum Event {
            $(
                $event {
                    $($arg_name: $arg_type,)*
                },
            )*
        }
    };
}

events! {
    Update, update {
        dt: f32,
    }
    ConnectPlayer, connect_player {
        player: SocketAddr,
    }
    DisconnectPlayer, disconnect_player {
        player: SocketAddr,
    }
    PlayerInput, player_input {
        player: SocketAddr,
        event:  input::Event,
    }
    LaunchMissile, launch_missile {
        missile: Missile,
    }
    ExplodeMissile, explode_missile {
        missile:   Entity,
        explosion: Explosion,
    }
    RemoveExplosion, remove_explosion {
        explosion: Entity,
    }
}
