pub mod background;


pub use self::background::Background;


pub struct FrameResources {
    pub output:  wgpu::SwapChainOutput,
    pub encoder: wgpu::CommandEncoder,
}
