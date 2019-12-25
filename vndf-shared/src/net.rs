pub mod message;


pub use self::message::Message;


use std::{
    io::{
        self,
        prelude::*,
    },
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpListener,
        TcpStream,
    },
    thread,
};

use log::{
    error,
    info,
};


pub const PORT: u16 = 34480;


pub struct Server;

impl Server {
    pub fn start() -> io::Result<Self> {
        let address  = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), PORT);
        let listener = TcpListener::bind(address)?;

        thread::spawn(|| listen(listener));

        Ok(Self)
    }
}


fn listen(listener: TcpListener) {
    for stream in listener.incoming() {
        if let Err(err) = handle_client(stream) {
            error!("Error accepting connection: {:?}", err);
        }
    }
}

fn handle_client(stream: io::Result<TcpStream>) -> Result<(), Error> {
    let mut stream = stream?;

    let addr = stream.peer_addr()?;
    info!("Connected: {}", addr);

    let mut buf = Vec::new();
    Message::Ping(0).serialize(&mut buf)?;

    stream.write_all(&buf)?;
    stream.flush()?;

    Ok(())
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
