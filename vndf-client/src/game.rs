pub mod config;
pub mod coords;
pub mod input;
pub mod net;
pub mod state;
pub mod transforms;


use std::{
    io,
    net::ToSocketAddrs,
};

use crate::shared::{
    math::Vec2,
    net::{
        client::Conn,
        msg,
    },
};

use self::{
    config::Config,
    coords::Screen,
    input::{
        Input,
        Transition,
    },
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

    pub fn handle_input(&mut self,
        input:       Input,
        screen_size: Screen<Vec2>,
    )
        -> Transition
    {
        let trans = self.input.handle(
            input,
            &self.state.camera,
            screen_size,
            &mut self.events,
        );

        for event in self.events.unsent() {
            self.conn.send(msg::FromClient::Action(event))
                .expect("Failed to send input event");
        }

        trans
    }
}


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Io(io::Error),
}
