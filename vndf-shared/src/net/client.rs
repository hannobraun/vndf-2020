use std::{
    io::prelude::*,
    net::TcpStream,
    thread,
};

use log::error;

use crate::net::{
    Error,
    Message,
};


pub struct Client;

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        thread::spawn(|| {
            if let Err(err) = receive(stream) {
                error!("Receive error: {:?}", err);
            }
        });

        Self
    }
}


fn receive(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = Vec::new();

    loop {
        let mut tmp = [0; 1024];
        
        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];

        buf.extend(read);

        while let Some(message) = Message::deserialize(&mut buf)? {
            let mut buf = Vec::new();
            message.serialize(&mut buf)?;

            stream.write_all(&buf)?;
            stream.flush()?;
        }
    }
}
