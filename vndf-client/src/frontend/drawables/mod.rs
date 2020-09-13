pub mod standard;
pub mod text;


pub use self::{
    standard::Standard,
    text::Text,
};


use std::io;

use super::{
    meshes::Meshes,
    shaders::{
        frag,
        vert,
    },
};


pub struct Drawables {
    pub explosion: Standard<vert::Simple, frag::Explosion>,
    pub orbit:     Standard<vert::Simple, frag::Orbit>,
    pub planet:    Standard<vert::Simple, frag::Planet>,
    pub ship:      Standard<vert::Simple, frag::Simple>,
    pub square:    Standard<vert::Simple, frag::Simple>,

    pub text: Text,
}

impl Drawables {
    pub fn new(
        device: &wgpu::Device,
        meshes: &Meshes,
        format: wgpu::TextureFormat,
    )
        -> Result<Self, Error>
    {
        let explosion = Standard::new(
            device,
            &meshes.square,
        )?;
        let orbit = Standard::new(
            device,
            &meshes.square,
        )?;
        let planet = Standard::new(
            device,
            &meshes.square,
        )?;
        let ship = Standard::new(
            device,
            &meshes.ship,
        )?;
        let square = Standard::new(
            device,
            &meshes.square,
        )?;

        let text = Text::new(device, format)
            .map_err(|err| Error::Text(err))?;

        Ok(
            Self {
                explosion,
                orbit,
                planet,
                ship,
                square,

                text,
            }
        )
    }
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Text(wgpu_glyph::ab_glyph::InvalidFont),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
