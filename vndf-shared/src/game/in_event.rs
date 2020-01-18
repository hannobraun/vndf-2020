use std::net::SocketAddr;

use hecs::Entity;

use crate::{
    game::entities::{
        Explosion,
        Missile,
    },
    input,
};

events! {
    InEvent {
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
        ExplodeCraft, explode_craft {
            craft:     Entity,
            explosion: Explosion,
        }
        CreateExplosion, create_explosion {
            explosion: Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: Entity,
        }
    }
}
