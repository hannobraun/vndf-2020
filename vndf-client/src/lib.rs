mod game;
mod frontends;
mod graphics;


pub use vndf_shared as shared;


use std::{
    net::ToSocketAddrs,
    str::FromStr,
};

use crate::game::Game;


pub fn start<A: ToSocketAddrs>(
    addr:     A,
    graphics: Graphics,
)
    -> Result<(), Error>
{
    let game = Game::init(addr)
        .map_err(Error::Game)?;

    frontends::start(game, graphics)
        .map_err(Error::Bespoke)
}


pub enum Graphics {
    Auto,
    DirectX11,
    DirectX12,
    Metal,
    OpenGl,
    Vulkan,
    WebGpu
}

impl FromStr for Graphics {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto"      => Ok(Self::Auto),
            "directx11" => Ok(Self::DirectX11),
            "directx12" => Ok(Self::DirectX12),
            "metal"     => Ok(Self::Metal),
            "opengl"    => Ok(Self::OpenGl),
            "vulkan"    => Ok(Self::Vulkan),
            "webgpu"    => Ok(Self::WebGpu),

            s => Err(format!("`{}` is not a valid graphics backend", s)),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    Bespoke(frontends::Error),
    Game(game::Error),
}
