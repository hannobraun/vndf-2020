use std::{
    io::{
        prelude::*,
        self,
    },
    iter,
    net::{
        SocketAddr,
        TcpStream,
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
    pub rx: Rx<In>,
    pub tx: Tx<Out>,
}

impl<In, Out> Conn<In, Out>
    where
        In:  Message + 'static,
        Out: Message + 'static,
{
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Self::from_stream(stream)
    }

    pub fn from_stream(stream: TcpStream) -> io::Result<Self> {
        let addr = stream.peer_addr()?;

        let (in_tx,  in_rx)  = channel();
        let (out_tx, out_rx) = channel();

        let stream_send    = stream.try_clone()?;
        let stream_receive = stream;

        thread::spawn(move ||
            if let Err(err) = send(stream_send, out_rx) {
                error!("Send error ({}): {:?}", addr, err);
            }
        );

        thread::spawn(move || {
            if let Err(err) = receive(stream_receive, in_tx) {
                error!("Receive error ({}) : {:?}", addr, err);
            }
        });

        Ok(
            Self {
                rx: Rx(in_rx),
                tx: Tx(out_tx),
            }
        )
    }

    pub fn incoming<'s>(&'s mut self)
        -> impl Iterator<Item=net::Result<In>> + 's
    {
        self.rx.incoming()
    }

    pub fn send(&mut self, message: Out) -> net::Result {
        self.tx.send(message)
    }

    pub fn disconnect(self) {
        // Nothing to do. The threads will end once our ends of the channels are
        // dropped. When the threads end, the streams will be dropped and the
        // connection will be closed.
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

fn receive<T>(mut stream: TcpStream, in_chan: Sender<T>) -> net::Result
    where T: Message
{
    let mut buf = Vec::new();

    loop {
        let mut tmp = [0; 1024];

        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];

        buf.extend(read);

        while let Some(message) = T::read(&mut buf)? {
            debug!("Received: {:?}", message);

            if let Err(SendError(_)) = in_chan.send(message) {
                // Other end has hung up. No need to keep this up.
                return Ok(())
            }
        }
    }
}


pub struct Rx<T>(Receiver<T>);

impl<T> Rx<T> {
    pub fn incoming<'s>(&'s mut self)
        -> impl Iterator<Item=net::Result<T>> + 's
    {
        iter::from_fn(move || {
            match self.0.try_recv() {
                Ok(event) => {
                    Some(Ok(event))
                }
                Err(TryRecvError::Empty) => {
                    None
                }
                Err(TryRecvError::Disconnected) => {
                    Some(Err(net::Error::ThreadFailed))
                }
            }
        })
    }
}

pub struct Tx<T>(Sender<T>);

impl<T> Tx<T> {
    pub fn send(&mut self, message: T) -> net::Result {
        self.0.send(message)?;
        Ok(())
    }
}
