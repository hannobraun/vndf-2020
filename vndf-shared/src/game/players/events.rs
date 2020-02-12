use std::net::SocketAddr;

use crate::{
    game::PlayerId,
    input,
};


pub struct InputHandled {
    pub addr: SocketAddr,
    pub seq:  u64,
}

pub struct PlayerConnected {
    pub addr:  SocketAddr,
    pub color: [f32; 3],
}

pub struct PlayerDisconnected {
    pub addr: SocketAddr,
}

pub struct PlayerCreated {
    pub id:   PlayerId,
    pub addr: SocketAddr,
}

pub struct PlayerInput {
    pub addr:  SocketAddr,
    pub event: input::Action,
}
