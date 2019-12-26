use std::{
    io,
    sync::mpsc,
};

use crate::net::msg;


pub type Result<T = ()> = std::result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Msg(msg::Error),

    /// Another thread failed
    ///
    /// We don't have any more information, we just know that the thread has
    /// dropped its end of the channel we've been using to communicate with it.
    /// The thread should have logged an error though.
    ThreadFailed,
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Error::Io(s), Error::Io(o))   => s.kind() == o.kind(),
            (Error::Msg(s), Error::Msg(o)) => s == o,
            _                              => false,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<msg::Error> for Error {
    fn from(err: msg::Error) -> Self {
        Self::Msg(err)
    }
}

impl<T> From<mpsc::SendError<T>> for Error {
    fn from(_: mpsc::SendError<T>) -> Self {
        Self::ThreadFailed
    }
}
