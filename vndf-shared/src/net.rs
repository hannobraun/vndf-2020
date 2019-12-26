pub mod client;
pub mod message;


pub use self::message::Message;


use std::{
    io,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpListener,
    },
    thread,
};

use log::error;

use self::client::Client;


pub const PORT: u16 = 34480;


pub struct Server;

impl Server {
    pub fn start_default() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), PORT))
    }

    pub fn start(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        thread::spawn(|| accept(listener));

        Ok(Self)
    }
}


fn accept(listener: TcpListener) {
    for stream in listener.incoming() {
        if let Err(err) = Client::new(stream) {
            error!("Error accepting connection: {:?}", err);
        }
    }
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Message(message::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<message::Error> for Error {
    fn from(err: message::Error) -> Self {
        Self::Message(err)
    }
}
