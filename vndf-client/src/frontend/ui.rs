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

use super::drawers::{
    DrawResources,
    Frame,
};


pub trait Ui {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        game:  &Game,
    )
        -> Result<(), ()>;

    fn handle_event(&mut self, _: &Event<()>, _: &Screen);
}
