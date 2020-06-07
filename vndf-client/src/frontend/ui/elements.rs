mod panel;
mod text;


pub use self::{
    panel::Panel,
    text::Text,
};


use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};


pub trait Element {
    fn size(&self, res: &mut DrawResources) -> graphics::Size;
    fn draw(self, res: &mut DrawResources, frame: &mut Frame);
}


pub fn text_panel(
    res:   &mut DrawResources,
    frame: &mut Frame,
    pos:   graphics::Pnt2,
    text:  &str,
)
    -> graphics::Size
{
    let text = Text::new(res, text, pos)
        .unwrap();

    let text_size = text.size(res);

    const PADDING: graphics::Scalar = 3.0;
    let padding = graphics::Size::new(
        PADDING * 2.0,
        PADDING * 2.0,
    );

    let panel_size = text_size + padding;

    Panel { pos: pos + text_size / 2.0, size: panel_size }
        .draw(res, frame);

    text.draw(res, frame);

    panel_size
}
