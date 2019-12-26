use std::io;

use crate::net::msg;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Msg(msg::Error),
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
