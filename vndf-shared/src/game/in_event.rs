use std::net::SocketAddr;

use hecs::Entity;

use crate::{
    game::entities::Missile,
    input,
};

events! {
    InEvent {
        Update, update {
            dt: f32,
        }
        ConnectPlayer, connect_player {
            player: SocketAddr,
            color:  [f32; 3],
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
        DeadEntity, dead_entity {
            entity: Entity,
        }
        Explosion, explosion {
            explosion: Entity,
        }
        RemoveExplosion, remove_explosion {
            explosion: Entity,
        }
    }
}
