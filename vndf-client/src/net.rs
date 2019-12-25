use std::{
    io,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpStream,
    },
};

use vndf_shared::net::PORT;


pub struct Conn;

impl Conn {
    pub fn connect() -> io::Result<Self> {
        let address = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), PORT);
        let _stream = TcpStream::connect(address)?;
        Ok(Self)
    }
}
