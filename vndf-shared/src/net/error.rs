use std::io;

use crate::net::comm;


#[derive(Debug)]
pub enum Error {
    Comm(comm::Error),
    Io(io::Error),
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Error::Io(s), Error::Io(o))     => s.kind() == o.kind(),
            (Error::Comm(s), Error::Comm(o)) => s == o,
            _                                => false,
        }
    }
}

impl From<comm::Error> for Error {
    fn from(err: comm::Error) -> Self {
        Self::Comm(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
