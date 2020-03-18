pub mod config;
pub mod input;
pub mod state;


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
