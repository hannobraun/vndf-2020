use std::{
    io,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpListener,
    },
    thread,
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
                        print!("Error retrieving peer address: {:?}\n", err);
                        continue;
                    }
                };
                print!("Connect: {}\n", addr);
            }
            Err(err) => {
                print!("Error accepting connection: {:?}\n", err);
                continue;
            }
        }
    }
}
