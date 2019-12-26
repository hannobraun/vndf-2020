use std::{
    io::{
        prelude::*,
        self,
    },
    marker::PhantomData,
    net::{
        SocketAddr,
        TcpStream,
    },
    sync::mpsc::{
        Receiver,
        RecvError,
        Sender,
        channel,
    },
    thread,
};

use log::{
    debug,
    error,
    info,
};

use crate::net::{
    self,
    Message,
    msg,
};


pub struct Conn<In, Out> {
    _in: PhantomData<In>,
    out: Sender<Out>,
}

impl<In, Out> Conn<In, Out>
    where
        Out: Message + 'static,
{
    pub fn accept(stream: io::Result<TcpStream>) -> io::Result<Self> {
        let stream = stream?;

        let addr = stream.peer_addr()?;
        info!("Connected: {}", addr);

        Self::new(stream)
    }

    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    fn new(stream: TcpStream) -> io::Result<Self> {
        let addr = stream.peer_addr()?;

        let (out_tx, out_rx) = channel();

        let stream_send    = stream.try_clone()?;
        let stream_receive = stream;

        thread::spawn(move ||
            if let Err(err) = send(stream_send, out_rx) {
                error!("Send error ({}): {:?}", addr, err);
            }
        );

        thread::spawn(move || {
            if let Err(err) = receive(stream_receive) {
                error!("Receive error ({}) : {:?}", addr, err);
            }
        });

        Ok(
            Self {
                _in: PhantomData,
                out: out_tx,
            }
        )
    }

    pub fn send(&mut self, message: Out) -> net::Result {
        self.out.send(message)?;
        Ok(())
    }
}


fn send<T>(mut stream: TcpStream, out: Receiver<T>) -> net::Result
    where T: Message
{
    let mut buf = Vec::new();

    loop {
        stream.write_all(&buf)?;
        buf.clear();

        match out.recv() {
            Ok(message) => {
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

fn receive(mut stream: TcpStream) -> net::Result {
    let mut buf = Vec::new();

    loop {
        let mut tmp = [0; 1024];

        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];

        buf.extend(read);

        while let Some(message) = msg::FromClient::read(&mut buf)? {
            debug!("Received: {:?}", message);

            let mut buf = Vec::new();
            msg::FromServer::Welcome.write(&mut buf)?;

            stream.write_all(&buf)?;
            stream.flush()?;
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Id(pub u64);
