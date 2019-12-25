use std::{
    io::{
        self,
        prelude::*,
    },
    iter,
    net::{
        Ipv6Addr,
        SocketAddr,
        TcpStream,
    },
    sync::mpsc::{
        Receiver,
        Sender,
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

use vndf_shared::net::{
    PORT,
    Message,
    message,
};


pub struct Conn {
    rx_receiver: Receiver<Message>,
}

impl Conn {
    pub fn connect() -> io::Result<Self> {
        let address = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), PORT);
        let stream  = TcpStream::connect(address)?;

        let (rx_sender, rx_receiver) = channel();

        thread::spawn(|| {
            if let Err(err) = receive(stream, rx_sender) {
                error!("Error receiving data: {:?}", err);
            }

            // If we reach this point, `receive` has failed. The world outside
            // this thread will notice this, because `rx_sender` has been
            // dropped.
        });

        Ok(
            Self {
                rx_receiver,
            }
        )
    }

    pub fn messages<'r>(&'r mut self)
        -> impl Iterator<Item=Result<Message, ReceiveError>> + 'r
    {
        iter::from_fn(move || {
            match self.rx_receiver.try_recv() {
                Ok(message) =>
                    Some(Ok(message)),
                Err(TryRecvError::Empty) =>
                    None,
                Err(TryRecvError::Disconnected) =>
                    Some(Err(ReceiveError)),
            }
        })
    }
}


fn receive(mut stream: TcpStream, sender: Sender<Message>)
    -> Result<(), Error>
{
    let mut buf = Vec::new();

    loop {
        trace!("Receiving. Buffer: {:?}", buf);

        let mut tmp = [0; 1024];

        let read = stream.read(&mut tmp)?;
        let read = &tmp[..read];
        buf.extend(read);

        while let Some(message) = Message::deserialize(&mut buf)? {
            debug!("Message received: {:?}", message);

            sender.send(message)
                .expect("Receiving end is disconnected");
        }
    }
}


pub struct ReceiveError;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Message(message::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<message::Error> for Error {
    fn from(err: message::Error) -> Self {
        Self::Message(err)
    }
}
