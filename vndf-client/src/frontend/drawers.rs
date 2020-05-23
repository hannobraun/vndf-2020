pub mod background;


pub use self::background::Background;


use crate::graphics::screen::Screen;


pub struct FrameResources {
    pub screen:  Screen,
    pub output:  wgpu::SwapChainOutput,
    pub encoder: wgpu::CommandEncoder,
}
