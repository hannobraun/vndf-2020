pub mod config;
pub mod input;
pub mod state;


use std::io;

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


#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Io(io::Error),
}
