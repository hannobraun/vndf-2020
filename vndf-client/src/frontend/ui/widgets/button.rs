use crate::{
    frontend::{
        drawers::{DrawResources, Frame},
        ui::{
            input::{Action, Input},
            traits::{DrawAt, DrawError, ProcessInputAt, Size},
        },
    },
    graphics,
};

use super::{text, TextPanel};

pub struct Button {
    panel: TextPanel,
    highlight: [f32; 4],
    action: Action,
}

impl Button {
    pub fn create(
        res: &mut DrawResources,
        text: String,
        action: Action,
        color: [f32; 4],
        highlight: [f32; 4],
    ) -> Result<Self, text::CreateError> {
        let mut panel = TextPanel::create(res, text)?;
        panel.panel_color(color);

        Ok(Self {
            panel,
            highlight,
            action,
        })
    }
}

impl DrawAt for Button {
    fn draw_at(
        &mut self,
        res: &mut DrawResources,
        frame: &mut Frame,
        pos: graphics::Pnt2,
    ) -> Result<(), DrawError> {
        self.panel.draw_at(res, frame, pos)
    }
}

impl ProcessInputAt for Button {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        let rect = graphics::Rect::new(pos, self.size());

        if let Some(cursor) = input.cursor {
            if rect.contains(cursor) {
                self.panel.panel_color(self.highlight);

                if input.click {
                    input.actions.push(self.action);
                }
            }
        }
    }
}

impl Size for Button {
    fn size(&self) -> graphics::Size {
        self.panel.size()
    }
}
