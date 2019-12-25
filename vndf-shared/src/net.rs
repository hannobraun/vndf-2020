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
            Ok(stream) => {
                let addr = match stream.peer_addr() {
                    Ok(address) => address,
                    Err(err) => {
                        error!("Error retrieving peer address: {:?}", err);
                        continue;
                    }
                };
                info!("Connect: {}", addr);
            }
            Err(err) => {
                error!("Error accepting connection: {:?}", err);
                continue;
            }
        }
    }
}
