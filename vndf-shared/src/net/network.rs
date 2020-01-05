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

use log::{
    error,
    info,
};

use crate::net::{
    self,
    conn,
    msg,
};


pub const PORT: u16 = 34480;


pub struct Network {
    addr:    SocketAddr,
    accept:  Receiver<(SocketAddr, ConnAdapter)>,
    receive: Receiver<Event>,
    clients: HashMap<SocketAddr, ConnAdapter>,
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

        let (accept_tx, accept_rx)   = channel();
        let (receive_tx, receive_rx) = channel();

        thread::spawn(|| accept(listener, accept_tx, receive_tx));

        Ok(
            Self {
                addr,
                accept:  accept_rx,
                receive: receive_rx,
                clients: HashMap::new(),
                remove:  VecDeque::new(),
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn clients(&self) -> impl Iterator<Item=SocketAddr> + '_ {
        self.clients.keys()
            .map(|&addr| addr)
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

        if let Err(err) = conn.0.send(message) {
            self.remove.push_back((addr, err));
            // No need to return the error. The user will get it via the
            // disconnect event.
        }
    }

    pub fn events<'s>(&'s mut self) -> impl Iterator<Item=Event> + 's {
        iter::from_fn(move || {
            if let Some((id, err)) = self.remove.pop_front() {
                self.clients.remove(&id);
                return Some(Event::Disconnect(id, err));
            }

            match self.receive.try_recv() {
                Ok(event) => {
                    return Some(event);
                }
                Err(TryRecvError::Empty) => {
                    ()
                }
                Err(TryRecvError::Disconnected) => {
                    // Can only happen if we have no connection threads _and_
                    // the accept thread ended, since that one has its own clone
                    // of the sender.
                    unreachable!(
                        "`accept` thread does not end while receiver exists"
                    );
                }
            }

            match self.accept.try_recv() {
                Ok((id, conn)) => {
                    self.clients.insert(id, conn);
                    return Some(Event::Connect(id));
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
    accept:   Sender<(SocketAddr, ConnAdapter)>,
    receive:  Sender<Event>,
) {
    for stream in listener.incoming() {
        let (conn, addr) = match ConnAdapter::accept(stream, receive.clone()) {
            Ok((conn, addr)) => {
                (conn, addr)
            }
            Err(err) => {
                error!("Error accepting connection: {:?}", err);
                continue;
            }
        };

        if let Err(SendError(_)) = accept.send((addr, conn)) {
            // Channel disconnected. This means the receiver has been dropped,
            // and we have no reason to keep this up.
            return;
        }
    }

    unreachable!("`listener.incoming()` does never yield `None`");
}


struct ConnAdapter(conn::Tx<msg::FromServer>);

impl ConnAdapter {
    pub fn accept(stream: io::Result<TcpStream>, receive: Sender<Event>)
        -> io::Result<(Self, SocketAddr)>
    {
        let stream = stream?;

        let conn = conn::Conn::from_stream(stream)?;
        info!("Connected: {}", conn.peer_addr);

        let mut rx   = conn.rx;
        let     tx   = conn.tx;
        let     addr = conn.peer_addr;

        thread::spawn(move || {
            loop {
                for message in rx.incoming() {
                    let message = match message {
                        Ok(message) => {
                            message
                        }
                        Err(err) => {
                            error!("Error receiving message: {:?}", err);

                            // We can ignore any channel errors here. The thread
                            // is ending anyway.
                            let event = Event::Disconnect(addr, err);
                            let _ = receive.send(event);

                            return;
                        }
                    };

                    let event = Event::Message(addr, message);

                    if let Err(SendError(_)) = receive.send(event) {
                        // Other hand has hung up. No need to keep this up.
                        return;
                    }
                }
            }
        });

        Ok((Self(tx), addr))
    }
}


#[derive(Debug, PartialEq)]
pub enum Event {
    Connect(SocketAddr),
    Disconnect(SocketAddr, net::Error),
    Message(SocketAddr, msg::FromClient),
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
