pub mod config;
pub mod input;
pub mod net;
pub mod state;
pub mod transforms;


use std::{
    io,
    net::ToSocketAddrs,
};

use crate::shared::net::client::Conn;

use self::{
    config::Config,
    net::input::Events,
    state::State,
};


pub struct Game {
    pub config: Config,
    pub conn:   Conn,
    pub events: Events,
    pub input:  input::Handler,
    pub state:  State,
}

impl Game {
    pub fn init<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        let config = Config::load()
            .map_err(|err| Error::Config(err))?;
        let conn = Conn::connect(addr)
            .map_err(|err| Error::Io(err))?;
        let events = Events::new();
        let input = input::Handler::new(config);
        let state = State::new();

        Ok(
            Self {
                config,
                conn,
                events,
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
