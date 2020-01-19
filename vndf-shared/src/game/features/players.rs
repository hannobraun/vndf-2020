use std::net::SocketAddr;

use crate::game::PlayerId;


pub struct NewPlayer {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}
