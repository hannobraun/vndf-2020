use std::net::SocketAddr;

use crate::{
    game::PlayerId,
    input,
};


pub struct PlayerConnected {
    pub addr:  SocketAddr,
    pub color: [f32; 3],
}

pub struct PlayerDisconnected {
    pub addr: SocketAddr,
}

pub struct PlayerItemCreated {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}

pub struct PlayerInput {
    pub addr:  SocketAddr,
    pub event: input::Event,
}