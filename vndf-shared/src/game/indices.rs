use std::{
    collections::HashMap,
    net::SocketAddr,
};

use crate::cgs::Handle;


pub struct Indices {
    pub players_by_address: HashMap<SocketAddr, Handle>,
}

impl Indices {
    pub fn new() -> Self {
        Self {
            players_by_address: HashMap::new(),
        }
    }
}
