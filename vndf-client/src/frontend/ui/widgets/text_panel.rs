use vndf_macros::ProcessInputAt;

use crate::{
    frontend::{
        drawers::{DrawResources, Frame},
        ui::traits::{DrawAt, DrawError, Size},
    },
    graphics,
};

use super::{text, Panel, Text};

#[derive(ProcessInputAt)]
pub struct TextPanel {
    text: Text,
    panel: Panel,
}

impl TextPanel {
    pub fn create(
        res: &mut DrawResources,
        text: String,
    ) -> Result<Self, text::CreateError> {
        const PADDING: graphics::Scalar = 3.0;
        let padding = graphics::Size::new(PADDING * 2.0, PADDING * 2.0);

        let text = Text::create(res, text)?;
        let panel = Panel::create(text.size() + padding);

        Ok(Self { text, panel })
    }

    pub fn panel_color(&mut self, color: [f32; 4]) {
        self.panel.color(color);
    }
}

impl Size for TextPanel {
    fn size(&self) -> graphics::Size {
        self.panel.size()
    }
}

impl DrawAt for TextPanel {
    fn draw_at(
        &mut self,
        res: &mut DrawResources,
        frame: &mut Frame,
        pos: graphics::Pnt2,
    ) -> Result<(), DrawError> {
        self.panel
            .draw_at(res, frame, pos + self.text.size() / 2.0)?;
        self.text.draw_at(res, frame, pos)?;

        Ok(())
    }
}
