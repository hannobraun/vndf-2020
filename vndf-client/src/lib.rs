mod draw;
mod game;
mod frontend;
mod graphics;
mod transforms;


pub use vndf_shared as shared;


use std::{
    io,
    net::ToSocketAddrs,
};

use crate::{
    game::{
        Game,
        config::{
            self,
            Config,
        },
        input::Input,
        state::State,
    },
    shared::net::client::Conn,
};


pub fn start<A: ToSocketAddrs>(addr: A) -> Result<(), Error> {
    let config = Config::load()
        .map_err(|err| Error::Config(err))?;
    let conn = Conn::connect(addr)
        .map_err(|err| Error::Io(err))?;
    let input = Input::new(config);
    let state = State::new();

    let game = Game { config, conn, input, state };

    frontend::ggez::start(game)
        .map_err(|err| Error::Ggez(err))
}


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Ggez(frontend::ggez::Error),
    Io(io::Error),
}
