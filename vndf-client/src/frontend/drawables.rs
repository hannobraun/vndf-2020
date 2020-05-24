mod standard;


pub use self::standard::Standard;


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
}

impl Drawables {
    pub fn new(device: &wgpu::Device, meshes: &Meshes)
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

        Ok(
            Self {
                explosion,
                orbit,
                panel,
                planet,
                ship,
            }
        )
    }
}
