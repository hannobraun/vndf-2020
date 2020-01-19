use std::net::SocketAddr;

use crate::game::PlayerId;


events!(
    OutEvent {
        Despawn, despawn {
            entity: hecs::Entity,
        }
        CreatePlayer, create_player {
            id:   PlayerId,
            addr: SocketAddr,
        }
    }
);
