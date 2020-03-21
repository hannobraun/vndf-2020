mod draw;
mod game;
mod frontends;
mod graphics;
mod transforms;


pub use vndf_shared as shared;


use std::{
    net::ToSocketAddrs,
    str::FromStr,
};

use crate::game::Game;


pub fn start<A: ToSocketAddrs>(addr: A, frontend: Frontend)
    -> Result<(), Error>
{
    let game = Game::init(addr)
        .map_err(Error::Game)?;

    match frontend {
        #[cfg(not(target_os = "windows"))]
        Frontend::Bespoke => {
            frontends::bespoke::start(game)
                .map_err(Error::Bespoke)
        }
        Frontend::Ggez => {
            frontends::ggez::start(game)
                .map_err(|err| Error::Ggez(err))
        }
    }
}


pub enum Frontend {
    #[cfg(not(target_os = "windows"))]
    Bespoke,
    Ggez,
}

impl FromStr for Frontend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(not(target_os = "windows"))]
            "bespoke" => Ok(Self::Bespoke),
            "ggez"    => Ok(Self::Ggez),
            s         => Err(format!("`{}` is not a valid frontend", s)),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    #[cfg(not(target_os = "windows"))]
    Bespoke(frontends::bespoke::Error),
    Game(game::Error),
    Ggez(frontends::ggez::Error),
}
