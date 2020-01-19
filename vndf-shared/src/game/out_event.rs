use std::net::SocketAddr;

use crate::game::PlayerId;


events!(
    OutEvent {
        Despawn, despawn {
            entity: hecs::Entity,
        }
        NewPlayer, new_player {
            id:   PlayerId,
            addr: SocketAddr,
        }
    }
);
