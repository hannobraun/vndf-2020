use std::{
    collections::HashMap,
    net::SocketAddr,
};


pub struct Indices {
    pub players: HashMap<SocketAddr, hecs::Entity>,
}

impl Indices {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}
