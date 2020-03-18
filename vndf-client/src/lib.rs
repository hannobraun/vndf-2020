mod draw;
mod game;
mod frontend;
mod graphics;
mod transforms;


pub use vndf_shared as shared;


use std::net::ToSocketAddrs;

use crate::game::Game;


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    let game = Game::init(addr)
        .map_err(Error::Game)?;

    frontend::ggez::start(game)
        .map_err(|err| Error::Ggez(err))
}


#[derive(Debug)]
pub enum Error {
    Game(game::Error),
    Ggez(frontend::ggez::Error),
}
