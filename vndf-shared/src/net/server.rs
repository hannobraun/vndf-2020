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


pub struct Server {
    addr:    SocketAddr,
    accept:  Receiver<(ConnId, ConnAdapter)>,
    receive: Receiver<Event>,
    conns:   HashMap<ConnId, ConnAdapter>,
    remove:  VecDeque<(ConnId, net::Error)>,
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
                conns:   HashMap::new(),
                remove:  VecDeque::new(),
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn send(&mut self, id: ConnId, message: msg::FromServer)
        -> Result<(), Error>
    {
        let conn = match self.conns.get_mut(&id) {
            Some(conn) => conn,
            None       => return Err(Error::NoSuchClient(id)),
        };

        if let Err(err) = conn.0.send(message) {
            self.remove.push_back((id, err));
            // No need to return the error. The user will get it via the
            // disconnect event.
        }

        Ok(())
    }

    pub fn events<'s>(&'s mut self) -> impl Iterator<Item=Event> + 's {
        iter::from_fn(move || {
            if let Some((id, err)) = self.remove.pop_front() {
                self.conns.remove(&id);
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
                    self.conns.insert(id, conn);
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
    accept:   Sender<(ConnId, ConnAdapter)>,
    receive:  Sender<Event>,
) {
    let mut next_id = 0;

    for stream in listener.incoming() {
        let id = ConnId(next_id);
        next_id += 1;

        let conn = match ConnAdapter::accept(id, stream, receive.clone()) {
            Ok(conn) => {
                conn
            }
            Err(err) => {
                error!("Error accepting connection: {:?}", err);
                continue;
            }
        };

        if let Err(SendError(_)) = accept.send((id, conn)) {
            // Channel disconnected. This means the receiver has been dropped,
            // and we have no reason to keep this up.
            return;
        }
    }

    unreachable!("`listener.incoming()` does never yield `None`");
}


struct ConnAdapter(conn::Tx<msg::FromServer>);

impl ConnAdapter {
    pub fn accept(
        id:      ConnId,
        stream:  io::Result<TcpStream>,
        receive: Sender<Event>,
    )
        -> io::Result<Self>
    {
        let stream = stream?;

        let conn = conn::Conn::from_stream(stream)?;
        info!("Connected: {}", conn.peer_addr);

        let mut rx = conn.rx;
        let     tx = conn.tx;

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
                            let event = Event::Disconnect(id, err);
                            let _ = receive.send(event);

                            break;
                        }
                    };

                    let event = Event::Message(id, message);

                    if let Err(SendError(_)) = receive.send(event) {
                        // Other hand has hung up. No need to keep this up.
                        break;
                    }
                }
            }
        });

        Ok(Self(tx))
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ConnId(pub u64);


#[derive(Debug, PartialEq)]
pub enum Event {
    Connect(ConnId),
    Disconnect(ConnId, net::Error),
    Message(ConnId, msg::FromClient),
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Net(net::Error),
    NoSuchClient(ConnId),
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
