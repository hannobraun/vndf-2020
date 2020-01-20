use std::net::SocketAddr;

use hecs::Entity;

use crate::{
    game::entities::Missile,
    input,
};

events! {
    InEvent {
        PlayerConnected, player_connected {
            addr:  SocketAddr,
            color: [f32; 3],
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
