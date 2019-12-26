use std::{
    io::{
        prelude::*,
        self,
    },
    net::{
        SocketAddr,
        TcpStream,
    },
    thread,
};

use log::{
    debug,
    error,
    info,
};

use crate::net::{
    self,
    Message as _,
    msg,
};


pub struct Conn;

impl Conn {
    pub fn accept(stream: io::Result<TcpStream>) -> io::Result<Self> {
        let stream = stream?;

        let addr = stream.peer_addr()?;
        info!("Connected: {}", addr);

        thread::spawn(|| {
            if let Err(err) = Self::receive(stream) {
                error!("Receive error: {:?}", err);
            }
        });

        Ok(Self)
    }

    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        TcpStream::connect(addr)?;

        Ok(Self)
    }

    fn receive(mut stream: TcpStream) -> net::Result {
        let mut buf = Vec::new();

        loop {
            let mut tmp = [0; 1024];

            let read = stream.read(&mut tmp)?;
            let read = &tmp[..read];

            buf.extend(read);

            while let Some(message) = msg::FromClient::read(&mut buf)? {
                debug!("Received: {:?}", message);

                let mut buf = Vec::new();
                msg::FromServer::Welcome.write(&mut buf)?;

                stream.write_all(&buf)?;
                stream.flush()?;
            }
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Id(pub u64);
