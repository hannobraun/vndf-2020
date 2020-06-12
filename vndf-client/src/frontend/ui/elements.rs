mod instructions;
mod panel;
mod text;
mod text_panel;


pub use self::{
    instructions::Instructions,
    panel::Panel,
    text::Text,
    text_panel::TextPanel,
};


use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};


pub trait Element {
    fn size(&self) -> graphics::Size;
    fn draw(self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    );
}
