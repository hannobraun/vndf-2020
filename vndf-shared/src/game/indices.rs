use std::{
    collections::HashMap,
    net::SocketAddr,
};


pub struct Indices {
    pub players_by_address: HashMap<SocketAddr, hecs::Entity>,
}

impl Indices {
    pub fn new() -> Self {
        Self {
            players_by_address: HashMap::new(),
        }
    }
}
