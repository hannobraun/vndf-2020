pub mod client;
pub mod comm;
pub mod server;


use std::{
    io,
    iter,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpListener,
        TcpStream,
    },
    sync::mpsc::{
        Receiver,
        Sender,
        SendError,
        TryRecvError,
        channel,
    },
    thread,
};

use log::error;

use self::client::Client;


pub const PORT: u16 = 34480;


pub struct Server {
    addr:    SocketAddr,
    accept:  Receiver<()>,
    next_id: u64,
}

impl Server {
    pub fn start_default() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), PORT))
    }

    pub fn start_local() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 0))
    }

    pub fn start(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        // Can't just use `addr`, as that could have a port number of zero, for
        // example, which won't be the actual port number.
        let addr = listener.local_addr()?;

        let (accept_tx, accept_rx) = channel();

        thread::spawn(|| Self::accept(listener, accept_tx));

        Ok(
            Self {
                addr,
                accept:  accept_rx,
                next_id: 0,
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn events<'s>(&'s mut self)
        -> impl Iterator<Item=Result<Event, Error>> + 's
    {
        iter::from_fn(move || {
            match self.accept.try_recv() {
                Ok(_conn) => {
                    let id = Id(self.next_id);
                    self.next_id += 1;

                    Some(Ok(Event::Connect(id)))
                }
                Err(TryRecvError::Empty) => {
                    None
                }
                Err(TryRecvError::Disconnected) => {
                    unreachable!(
                        "`accept` thread does not end while receiver exists"
                    );

                }
            }
        })
    }

    fn accept(listener: TcpListener, accept: Sender<()>) {
        for stream in listener.incoming() {
            if let Err(err) = Client::new(stream) {
                error!("Error accepting connection: {:?}", err);
            }

            if let Err(SendError(_)) = accept.send(()) {
                // Channel disconnected. This means the receiver has been
                // dropped, and we have no reason to keep this up.
                return;
            }
        }

        unreachable!("`listener.incoming()` does never yield `None`");
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Connect(Id),
}


#[derive(Debug, Eq, PartialEq)]
pub struct Id(pub u64);


pub struct Conn;

impl Conn {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        TcpStream::connect(addr)?;

        Ok(Self)
    }
}


#[derive(Debug)]
pub enum Error {
    Comm(comm::Error),
    Io(io::Error),
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Error::Io(s), Error::Io(o))     => s.kind() == o.kind(),
            (Error::Comm(s), Error::Comm(o)) => s == o,
            _                                => false,
        }
    }
}

impl From<comm::Error> for Error {
    fn from(err: comm::Error) -> Self {
        Self::Comm(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
