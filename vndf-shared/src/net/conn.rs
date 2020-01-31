use std::{
    io::{
        prelude::*,
        self,
    },
    iter,
    net::{
        SocketAddr,
        TcpStream,
        ToSocketAddrs,
    },
    sync::mpsc::{
        Receiver,
        RecvError,
        Sender,
        SendError,
        TryRecvError,
        channel,
    },
    thread,
};

use log::{
    debug,
    error,
    trace,
};

use crate::net::{
    self,
    Message,
};


pub struct Conn<In, Out> {
    rx: Receiver<Option<In>>,
    tx: Sender<Out>,

    pub local_addr: SocketAddr,
    pub peer_addr:  SocketAddr,
}

impl<In, Out> Conn<In, Out>
    where
        In:  Message + 'static,
        Out: Message + 'static,
{
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Self::from_stream(stream, true)
    }

    pub fn from_stream(stream: TcpStream, errors_are_critical: bool)
        -> io::Result<Self>
    {
        stream.set_nodelay(true)?;

        let local_addr = stream.local_addr()?;
        let peer_addr  = stream.peer_addr()?;

        let (in_tx,  in_rx)  = channel();
        let (out_tx, out_rx) = channel();

        let stream_send    = stream.try_clone()?;
        let stream_receive = stream;

        thread::spawn(move ||
            if let Err(err) = send(stream_send, out_rx) {
                if errors_are_critical {
                    error!("Send error ({}): {:?}", peer_addr, err);
                }
                else {
                    debug!("Send error ({}): {:?}", peer_addr, err);
                }
            }
        );

        thread::spawn(move || {
            if let Err(err) = receive(stream_receive, in_tx) {
                if errors_are_critical {
                    error!("Receive error ({}) : {:?}", peer_addr, err);
                }
                else {
                    debug!("Receive error ({}) : {:?}", peer_addr, err);
                }
            }
        });

        Ok(
            Self {
                rx: in_rx,
                tx: out_tx,
                local_addr,
                peer_addr,
            }
        )
    }

    pub fn incoming<'s>(&'s mut self)
        -> impl Iterator<Item=net::Result<In>> + 's
    {
        iter::from_fn(move || {
            loop {
                match self.rx.try_recv() {
                    Ok(Some(event)) => {
                        return Some(Ok(event));
                    }
                    Ok(None) => {
                        // Just a ping from the receive thread. Real messages
                        // might be coming after it.
                        continue;
                    }
                    Err(TryRecvError::Empty) => {
                        return None;
                    }
                    Err(TryRecvError::Disconnected) => {
                        return Some(Err(net::Error::ThreadFailed));
                    }
                }
            }
        })
    }

    pub fn send(&mut self, message: Out) -> net::Result {
        self.tx.send(message)?;
        Ok(())
    }

    pub fn disconnect(self) {
        // Nothing to do. The threads will end once our ends of the channels are
        // dropped. When the threads end, the streams will be dropped and the
        // connection will be closed.
    }
}


fn receive<T>(mut stream: TcpStream, in_chan: Sender<Option<T>>) -> net::Result
    where T: Message
{
    let mut buf = Vec::new();

    loop {
        trace!("Starting receive loop");

        let mut tmp = [0; 1024];

        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];

        buf.extend(read);

        let mut message = None;

        loop {
            // We need to try this at least once, no matter if we have a message
            // or not. Otherwise, we might never notice, if the other end has
            // hung up.
            if let Err(SendError(_)) = in_chan.send(message) {
                // Other end has hung up. No need to keep this up.
                return Ok(())
            }

            message = T::read(&mut buf)?;

            if message.is_none() {
                // No more message to read. No need to keep sending crap through
                // the channel.
                break;
            }
        }
    }
}

fn send<T>(mut stream: TcpStream, out_chan: Receiver<T>) -> net::Result
    where T: Message
{
    let mut buf = Vec::new();

    loop {
        trace!("Starting send loop: {:?}", buf);

        stream.write_all(&buf)?;
        buf.clear();

        match out_chan.recv() {
            Ok(message) => {
                debug!("Writing message: {:?}", message);
                message.write(&mut buf)?;
            }
            Err(RecvError) => {
                // This means the other end has hung up. No need to continue
                // here.
                return Ok(());
            }
        }
    }
}
