mod standard;
mod text;


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
    pub panel:     Standard<vert::Simple, frag::Simple>,
    pub planet:    Standard<vert::Simple, frag::Planet>,
    pub ship:      Standard<vert::Simple, frag::Simple>,

    pub text: Text,
}

impl Drawables {
    pub fn new(
        device: &wgpu::Device,
        meshes: &Meshes,
        format: wgpu::TextureFormat,
    )
        -> Result<Self, io::Error>
    {
        let explosion = Standard::new(
            device,
            &meshes.square,
        )?;
        let orbit = Standard::new(
            device,
            &meshes.square,
        )?;
        let panel = Standard::new(
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

        let text = Text::new(device, format)?;

        Ok(
            Self {
                explosion,
                orbit,
                panel,
                planet,
                ship,

                text,
            }
        )
    }
}
