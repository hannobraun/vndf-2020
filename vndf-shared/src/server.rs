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
        Ok(Self::new(net::Server::start_default()?))
    }

    pub fn start_local() -> net::Result<Self> {
        Ok(Self::new(net::Server::start_local()?))
    }

    fn new(server: net::Server) -> Self {
        Self {
            server,
            events: Vec::new(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.server.addr()
    }

    pub fn update(&mut self) {
        self.events.extend(self.server.events());

        for event in self.events.drain(..) {
            match event {
                server::Event::Message(id, msg::FromClient::Hello) => {
                    self.server.send(id, msg::FromServer::Welcome);
                }
                server::Event::Message(id, msg::FromClient::Input(input)) => {
                    self.server.send(id, msg::FromServer::Input(input));
                }
                _ => (),
            }
        }
    }
}
