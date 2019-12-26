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
        SendError,
        channel,
    },
    thread,
};

use log::{
    debug,
    error,
    info,
    trace,
};

use crate::net::{
    self,
    Message,
};


pub struct Conn<In, Out> {
    _in:      PhantomData<In>,
    out_chan: Sender<Out>,
}

impl<In, Out> Conn<In, Out>
    where
        In:  Message + 'static,
        Out: Message + 'static,
{
    pub fn accept(stream: io::Result<TcpStream>, in_chan: Sender<In>)
        -> io::Result<Self>
    {
        let stream = stream?;

        let addr = stream.peer_addr()?;
        info!("Connected: {}", addr);

        Self::new(stream, Some(in_chan))
    }

    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream, None)
    }

    fn new(stream: TcpStream, in_chan: Option<Sender<In>>) -> io::Result<Self> {
        let addr = stream.peer_addr()?;

        let (in_tx, _) = match in_chan {
            Some(in_chan) => {
                (in_chan, None)
            }
            None => {
                let (in_tx, in_rx) = channel();
                (in_tx, Some(in_rx))
            }
        };
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
                _in:      PhantomData,
                out_chan: out_tx,
            }
        )
    }

    pub fn send(&mut self, message: Out) -> net::Result {
        self.out_chan.send(message)?;
        Ok(())
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
