pub mod background;
pub mod orbit;
pub mod planet;


pub use self::{
    background::draw_background,
    orbit::draw_orbit,
    planet::draw_planet,
};


use crate::graphics::screen::Screen;

use super::drawables::Drawables;


pub struct DrawResources {
    pub device:    wgpu::Device,
    pub drawables: Drawables,
}


pub struct Frame {
    pub screen:  Screen,
    pub output:  wgpu::SwapChainOutput,
    pub encoder: wgpu::CommandEncoder,
}
