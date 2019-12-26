pub mod client;
pub mod message;


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
use serde::{
    Deserialize,
    Serialize,
};

use self::client::Client;


pub const PORT: u16 = 34480;


pub struct Server;

impl Server {
    pub fn start_default() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), PORT))
    }

    pub fn start(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        thread::spawn(|| Self::accept(listener));

        Ok(Self)
    }

    fn accept(listener: TcpListener) -> ! {
        for stream in listener.incoming() {
            if let Err(err) = Client::new(stream) {
                error!("Error accepting connection: {:?}", err);
            }
        }

        unreachable!("`listener.incoming()` does never yield `None`");
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Ping(u64),
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Message(message::Error),
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Error::Io(s), Error::Io(o))           => s.kind() == o.kind(),
            (Error::Message(s), Error::Message(o)) => s == o,
            _                                      => false,
        }
    }
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
