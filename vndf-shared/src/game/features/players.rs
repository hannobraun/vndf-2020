use std::net::SocketAddr;

use crate::game::PlayerId;


pub struct PlayerEntityCreated {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}
