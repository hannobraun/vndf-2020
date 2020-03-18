mod draw;
mod game;
mod frontend;
mod graphics;
mod transforms;


pub use vndf_shared as shared;


use std::net::ToSocketAddrs;


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    frontend::ggez::start(addr)
        .map_err(|err| Error::Ggez(err))
}


#[derive(Debug)]
pub enum Error {
    Ggez(frontend::ggez::Error),
}
