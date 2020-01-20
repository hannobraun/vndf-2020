use std::net::SocketAddr;

use crate::game::PlayerId;


pub struct PlayerConnected {
    pub addr:  SocketAddr,
    pub color: [f32; 3],
}

pub struct PlayerEntityCreated {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}
