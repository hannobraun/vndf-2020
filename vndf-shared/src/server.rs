use std::net::SocketAddr;

use crate::net::{
    self,
    msg,
    server,
};


pub struct Server {
    server: net::Server,
    events: Vec<server::Event>,
}

impl Server {
    pub fn start_default() -> net::Result<Self> {
        Ok(
            Self {
                server: net::Server::start_default()?,
                events: Vec::new(),
            }
        )
    }

    pub fn start_local() -> net::Result<Self> {
        Ok(
            Self {
                server: net::Server::start_local()?,
                events: Vec::new(),
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.server.addr()
    }

    pub fn update(&mut self) {
        self.events.extend(self.server.events());

        for event in self.events.drain(..) {
            match event {
                server::Event::Message(id, msg::FromClient::Hello) => {
                    // Ignore error. The client will be disconnected
                    // automatically, in case of errors.
                    let _ = self.server.send(id, msg::FromServer::Welcome);
                }
                _ => (),
            }
        }
    }
}
