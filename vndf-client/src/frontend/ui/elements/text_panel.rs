use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::Element,
    },
    graphics,
};

use super::{
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
    )
        -> Result<Self, NoBoundsError>
    {
        const PADDING: graphics::Scalar = 3.0;
        let padding = graphics::Size::new(
            PADDING * 2.0,
            PADDING * 2.0,
        );

        let text = Text::new(res, text)?;

        let panel = Panel {
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

impl Element for TextPanel<'_> {
    fn size(&self) -> graphics::Size {
        self.panel.size
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.panel.draw(res, frame, pos + self.text.size() / 2.0);
        self.text.draw(res, frame, pos);
    }
}
