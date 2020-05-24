pub mod background;


pub use self::background::draw_background;


use crate::graphics::screen::Screen;


pub struct Frame {
    pub screen:  Screen,
    pub output:  wgpu::SwapChainOutput,
    pub encoder: wgpu::CommandEncoder,
}
