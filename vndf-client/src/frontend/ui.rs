pub mod basic;
pub mod conrod;


pub use self::{
    basic::Basic,
    conrod::Conrod,
};


use winit::event::Event;

use crate::{
    game::Game,
    graphics::screen::Screen,
};


pub trait Ui {
    fn draw(&mut self,
        device:  &wgpu::Device,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        game:    &Game,
        screen:  &Screen,
    )
        -> Result<(), ()>;

    fn handle_event(&mut self, event: &Event<()>);
}
