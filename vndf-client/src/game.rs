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

use log::{
    debug,
    error,
};

use crate::shared::{
    net::{
        client::Conn,
        msg,
    },
    world::math::Vec2,
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

    pub fn update(&mut self) -> Result<(), ()> {
        for message in self.conn.incoming() {
            match message {
                Ok(msg::FromServer::Ping) => {
                    // This message is just for testing purposes. Nothing to do
                    // here.
                }
                Ok(msg::FromServer::Welcome(id)) => {
                    self.state.own_id = Some(id);
                }
                Ok(msg::FromServer::UpdateComponent(component)) => {
                    debug!("Update component: {:?}", component);
                    self.state.update_component(component);
                }
                Ok(msg::FromServer::RemoveComponent(handle)) => {
                    self.state.remove_component(&handle);
                }
                Ok(msg::FromServer::InputHandled { seq }) => {
                    self.events.handled(seq);
                }
                Ok(msg::FromServer::Diagnostics(diagnostics)) => {
                    self.state.diagnostics = Some(diagnostics);
                }
                Err(err) => {
                    error!("Connection error: {:?}", err);
                    return Err(());
                }
            }
        }

        Ok(())
    }
}


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Io(io::Error),
}
