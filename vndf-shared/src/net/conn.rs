use std::{
    io::{
        prelude::*,
        self,
    },
    marker::PhantomData,
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


pub struct Conn<In, Out> {
    _in:  PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<In, Out> Conn<In, Out> {
    pub fn accept(stream: io::Result<TcpStream>) -> io::Result<Self> {
        let stream = stream?;

        let addr = stream.peer_addr()?;
        info!("Connected: {}", addr);

        Self::new(stream)
    }

    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    fn new(stream: TcpStream) -> io::Result<Self> {
        thread::spawn(|| {
            if let Err(err) = receive(stream) {
                error!("Receive error: {:?}", err);
            }
        });

        Ok(
            Self {
                _in:  PhantomData,
                _out: PhantomData,
            }
        )
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Id(pub u64);


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
