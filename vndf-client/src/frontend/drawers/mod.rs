pub mod background;
pub mod explosion;
pub mod grid;
pub mod orbit;
pub mod planet;
pub mod ship;

pub use self::{
    background::draw_background, explosion::draw_explosion, grid::draw_grid,
    orbit::draw_orbit, planet::draw_planet, ship::draw_ship,
};

use crate::graphics::screen::Screen;

use super::drawables::Drawables;

pub struct DrawResources {
    pub device: wgpu::Device,
    pub drawables: Drawables,
}

pub struct Frame {
    pub screen: Screen,
    pub output: wgpu::SwapChainTexture,
    pub encoder: wgpu::CommandEncoder,
}
