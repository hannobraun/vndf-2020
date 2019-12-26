use std::{
    io,
    iter,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpListener,
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
    msg,
};


pub const PORT: u16 = 34480;


pub struct Server {
    addr:    SocketAddr,
    accept:  Receiver<()>,
    receive: Receiver<msg::FromClient>,
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
                next_id: 0,
            }
        )
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn events<'s>(&'s mut self)
        -> impl Iterator<Item=net::Result<Event>> + 's
    {
        iter::from_fn(move || {
            match self.receive.try_recv() {
                Ok(message) => {
                    return Some(Ok(Event::Message(message)));
                }
                Err(TryRecvError::Empty) => {
                    ()
                }
                Err(TryRecvError::Disconnected) => {
                    return Some(Err(net::Error::ThreadFailed));
                }
            }

            match self.accept.try_recv() {
                Ok(_conn) => {
                    let id = ConnId(self.next_id);
                    self.next_id += 1;

                    return Some(Ok(Event::Connect(id)));
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

            // If we returned nothing by thing point, there's nothing to be
            // returned.
            None
        })
    }
}


fn accept(
    listener: TcpListener,
    accept:   Sender<()>,
    receive:  Sender<msg::FromClient>,
) {
    for stream in listener.incoming() {
        if let Err(err) = Conn::accept(stream, receive.clone()) {
            error!("Error accepting connection: {:?}", err);
        }

        if let Err(SendError(_)) = accept.send(()) {
            // Channel disconnected. This means the receiver has been dropped,
            // and we have no reason to keep this up.
            return;
        }
    }

    unreachable!("`listener.incoming()` does never yield `None`");
}


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ConnId(pub u64);


#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Connect(ConnId),
    Message(msg::FromClient),
}


pub type Conn = net::Conn<msg::FromClient, msg::FromServer>;
