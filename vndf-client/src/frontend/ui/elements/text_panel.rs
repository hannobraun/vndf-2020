use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};

use super::{
    Element,
    Panel,
    Text,
    text::NoBoundsError,
};


pub struct TextPanel<'r> {
    text:  Text<'r>,
    panel: Panel,
}

impl<'r> TextPanel<'r> {
    pub fn new(
        res:  &mut DrawResources,
        text: &'r str,
        pos:  graphics::Pnt2,
    )
        -> Result<Self, NoBoundsError>
    {
        const PADDING: graphics::Scalar = 3.0;
        let padding = graphics::Size::new(
            PADDING * 2.0,
            PADDING * 2.0,
        );

        let text = Text::new(res, text, pos)?;

        let panel = Panel {
            pos:  pos + text.size() / 2.0,
            size: text.size() + padding,
        };

        Ok(
            Self {
                text,
                panel,
            }
        )
    }
}

impl<'r> Element for TextPanel<'r> {
    fn size(&self) -> graphics::Size {
        self.panel.size
    }

    fn draw(self, res: &mut DrawResources, frame: &mut Frame) {
        self.panel.draw(res, frame);
        self.text.draw(res, frame);
    }
}
