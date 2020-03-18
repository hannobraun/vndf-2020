pub mod config;
pub mod input;
pub mod state;


use crate::shared::net::client::Conn;

use self::{
    config::Config,
    input::Input,
};


pub struct Game {
    pub config: Config,
    pub conn:   Conn,
    pub input:  Input,
}
