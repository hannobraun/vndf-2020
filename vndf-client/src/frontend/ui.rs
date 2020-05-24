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
    drawers::Frame,
};


pub trait Ui {
    fn draw(&mut self,
        device:    &wgpu::Device,
        frame:     &mut Frame,
        drawables: &mut Drawables,
        game:      &Game,
    )
        -> Result<(), ()>;

    fn handle_event(&mut self, _: &Event<()>, _: &Screen);
}
