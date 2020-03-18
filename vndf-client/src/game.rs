pub mod config;
pub mod input;
pub mod state;


use std::{
    io,
    net::ToSocketAddrs,
};

use crate::shared::net::client::Conn;

use self::{
    config::Config,
    input::Input,
    state::State,
};


pub struct Game {
    pub config: Config,
    pub conn:   Conn,
    pub input:  Input,
    pub state:  State,
}

impl Game {
    pub fn init<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        let config = Config::load()
            .map_err(|err| Error::Config(err))?;
        let conn = Conn::connect(addr)
            .map_err(|err| Error::Io(err))?;
        let input = Input::new(config);
        let state = State::new();

        Ok(
            Self {
                config,
                conn,
                input,
                state,
            }
        )
    }
}


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Io(io::Error),
}
