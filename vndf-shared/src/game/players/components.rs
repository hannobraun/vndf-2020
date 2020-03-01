use std::net::SocketAddr;

use serde::{
    Deserialize,
    Serialize,
};

use crate::game::players::PlayerId;


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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
