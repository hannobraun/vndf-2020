mod config;
mod draw;
mod game;
mod frontend;
mod graphics;
mod input;
mod transforms;


pub use vndf_shared as shared;


use std::net::ToSocketAddrs;


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    frontend::ggez::start(addr)
        .map_err(|err| Error(err))
}


#[derive(Debug)]
pub struct Error(frontend::ggez::Error);
