use std::net::SocketAddr;

use hecs::Entity;

use crate::{
    game::entities::Missile,
    input,
};

events! {
    InEvent {
        PlayerInput, player_input {
            addr:  SocketAddr,
            event: input::Event,
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
