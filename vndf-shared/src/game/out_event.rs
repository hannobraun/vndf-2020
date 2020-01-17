use std::net::SocketAddr;

use hecs::Entity;

use crate::game::PlayerId;


events!(
    OutEvent {
        Despawn, despawn {
            entity: Entity,
        }
        CreatePlayer, create_player {
            id:   PlayerId,
            addr: SocketAddr,
        }
    }
);
