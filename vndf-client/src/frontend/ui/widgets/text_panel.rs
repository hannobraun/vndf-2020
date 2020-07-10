use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Widget,
        },
    },
    graphics,
};

use super::{
    Panel,
    Text,
    text::NoBoundsError,
};


pub struct TextPanel {
    text:  Text,
    panel: Panel,
}

impl TextPanel {
    pub fn new(
        res:  &mut DrawResources,
        text: String,
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

impl Widget for TextPanel {
    fn size(&self) -> graphics::Size {
        self.panel.size
    }
}

impl DrawAt for TextPanel {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.panel.draw_at(res, frame, pos + self.text.size() / 2.0);
        self.text.draw_at(res, frame, pos);
    }
}
