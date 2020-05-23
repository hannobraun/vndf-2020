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

use super::{
    drawables::Drawables,
    drawers::FrameResources,
};


pub trait Ui {
    fn draw(&mut self,
        device:    &wgpu::Device,
        res:       &mut FrameResources,
        drawables: &mut Drawables,
        game:      &Game,
    )
        -> Result<(), ()>;

    fn handle_event(&mut self, _: &Event<()>, _: &Screen);
}
