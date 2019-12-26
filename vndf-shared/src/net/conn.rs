use std::{
    io,
    net::{
        SocketAddr,
        TcpStream,
    },
};


pub struct Conn;

impl Conn {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        TcpStream::connect(addr)?;

        Ok(Self)
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Id(pub u64);
