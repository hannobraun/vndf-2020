use std::{
    collections::{
        HashMap,
        VecDeque,
    },
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

use crate::net::{
    self,
    conn,
    msg,
};


pub const PORT: u16 = 34480;


pub struct Network {
    addr:    SocketAddr,
    accept:  Receiver<Conn>,
    clients: HashMap<SocketAddr, Conn>,
    remove:  VecDeque<(SocketAddr, net::Error)>,
}

impl Network {
    pub fn start_default() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), PORT))
    }

    pub fn start_local() -> io::Result<Self> {
        Self::start(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 0))
    }

    pub fn start(addr: SocketAddr) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;

        // We can't just use `addr`, as that could have a port number of `0`,
        // for example, which won't be the actual port number we're listening
        // on.
        let addr = listener.local_addr()?;

        let (accept_tx, accept_rx) = channel();

        thread::spawn(|| accept(listener, accept_tx));

        Ok(
            Self {
                addr,
                accept:  accept_rx,
                clients: HashMap::new(),
                remove:  VecDeque::new(),
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn send(&mut self, addr: SocketAddr, message: msg::FromServer) {
        let conn = match self.clients.get_mut(&addr) {
            Some(conn) => conn,

            // Just return, if this client doesn't exist. We could return an
            // error here, of course, but I don't think that is an error that
            // could actually be handled in a sensible way.
            //
            // If the client doesn't exist because of a bug in the program, then
            // we'd like to have a panic. The caller can't just `unwrap though
            // as the client could also have just been removed, before the
            // caller had a chance to handle that event.
            //
            // So there's really nothing the caller could do with this error,
            // except ignore it. Let's save the caller that bit of trouble and
            // just make this a no-op.
            None => return,
        };

        if let Err(err) = conn.send(message) {
            self.remove.push_back((addr, err));
            // No need to return the error. The user will get it via the
            // disconnect event.
        }
    }

    pub fn events<'s>(&'s mut self) -> impl Iterator<Item=Event> + 's {
        iter::from_fn(move || {
            if let Some((id, err)) = self.remove.pop_front() {
                self.clients.remove(&id);
                return Some(Event::Error(id, err));
            }

            for (&addr, conn) in &mut self.clients {
                match conn.incoming().next() {
                    Some(Ok(message)) => {
                        return Some(Event::Message(addr, message));
                    }
                    Some(Err(err)) => {
                        self.remove.push_back((addr, err));
                    }
                    None => {
                        // Do nothing. The next loop iteration will look at
                        // another client.
                    }
                }
            }

            match self.accept.try_recv() {
                Ok(conn) => {
                    self.clients.insert(conn.peer_addr, conn);
                }
                Err(TryRecvError::Empty) => {
                    ()
                }
                Err(TryRecvError::Disconnected) => {
                    unreachable!(
                        "`accept` thread does not end while receiver exists"
                    );
                }
            }

            // If we returned nothing by this point, there's nothing to be
            // returned.
            None
        })
    }
}


fn accept(
    listener: TcpListener,
    accept:   Sender<Conn>,
) {
    for stream in listener.incoming() {
        let conn = match accept_conn(stream) {
            Ok(conn) => {
                conn
            }
            Err(err) => {
                error!("Error accepting connection: {:?}", err);
                continue;
            }
        };

        if let Err(SendError(_)) = accept.send(conn) {
            // Channel disconnected. This means the receiver has been dropped,
            // and we have no reason to keep this up.
            return;
        }
    }

    unreachable!("`listener.incoming()` does never yield `None`");
}

fn accept_conn(stream: io::Result<TcpStream>) -> io::Result<Conn> {
    let stream = stream?;
    Conn::from_stream(stream)
}


pub type Conn = conn::Conn<msg::FromClient, msg::FromServer>;


#[derive(Debug, PartialEq)]
pub enum Event {
    Message(SocketAddr, msg::FromClient),
    Error(SocketAddr, net::Error),
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Net(net::Error),
    NoSuchClient(SocketAddr),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<net::Error> for Error {
    fn from(err: net::Error) -> Self {
        Self::Net(err)
    }
}
