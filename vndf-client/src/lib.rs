mod draw;
mod game;
mod frontend;
mod graphics;
mod transforms;


pub use vndf_shared as shared;


use std::net::ToSocketAddrs;

use crate::game::Game;


pub fn start<A: ToSocketAddrs>(addr: A, frontend: Frontend)
    -> Result<(), Error>
{
    let game = Game::init(addr)
        .map_err(Error::Game)?;

    match frontend {
        Frontend::Ggez => {
            frontend::ggez::start(game)
                .map_err(|err| Error::Ggez(err))
        }
    }
}


pub enum Frontend {
    Ggez,
}


#[derive(Debug)]
pub enum Error {
    Game(game::Error),
    Ggez(frontend::ggez::Error),
}
