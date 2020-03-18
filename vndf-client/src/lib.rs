mod config;
mod draw;
mod game;
mod frontend;
mod graphics;
mod input;
mod transforms;


pub use vndf_shared as shared;

pub use self::frontend::ggez::Error;


use std::net::ToSocketAddrs;


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    frontend::ggez::start(addr)
}
