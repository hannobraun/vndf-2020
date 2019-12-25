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
        match stream {
            Ok(mut stream) => {
                let addr = match stream.peer_addr() {
                    Ok(address) => address,
                    Err(err) => {
                        error!("Error retrieving peer address: {:?}", err);
                        continue;
                    }
                };

                info!("Connected: {}", addr);

                let mut buf = Vec::new();
                Message::Ping(0).serialize(&mut buf)
                    .expect("Failed to serialize message");

                stream.write_all(&buf)
                    .expect("Failed to write ping");
                stream.flush()
                    .expect("Failed to flush ping");
            }
            Err(err) => {
                error!("Error accepting connection: {:?}", err);
                continue;
            }
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
