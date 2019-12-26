use std::{
    io::{
        self,
        prelude::*,
    },
    net::TcpStream,
    thread,
};

use log::{
    debug,
    error,
    info,
};

use crate::{
    input,
    net::{
        Error,
        Message as _,
        msg,
    },
};


pub struct Client;

impl Client {
    pub fn new(stream: io::Result<TcpStream>) -> io::Result<Self> {
        let stream = stream?;

        let addr = stream.peer_addr()?;
        info!("Connected: {}", addr);

        thread::spawn(|| {
            if let Err(err) = receive(stream) {
                error!("Receive error: {:?}", err);
            }
        });

        Ok(Self)
    }
}


fn receive(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = Vec::new();

    loop {
        let mut tmp = [0; 1024];
        
        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];

        buf.extend(read);

        while let Some(message) = input::Event::read(&mut buf)? {
            debug!("Received: {:?}", message);

            let mut buf = Vec::new();
            msg::FromServer::Welcome.write(&mut buf)?;

            stream.write_all(&buf)?;
            stream.flush()?;
        }
    }
}
