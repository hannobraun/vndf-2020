use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::{
            self,
            Size as _,
        },
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

impl<'r> elements::Size for TextPanel<'r> {
    fn size(&self) -> graphics::Size {
        self.panel.size
    }
}

impl<'r> Element for TextPanel<'r> {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.panel.draw(res, frame, pos + self.text.size() / 2.0);
        self.text.draw(res, frame, pos);
    }
}
