use std::net::SocketAddr;

use crate::game::players::PlayerId;


#[derive(Clone, Copy)]
pub struct Player {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}

impl Player {
    pub fn new(id: PlayerId, addr: SocketAddr) -> Self {
        Self {
            id,
            addr,
        }
    }
}
