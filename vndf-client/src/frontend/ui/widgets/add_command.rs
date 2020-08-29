use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::{
                Input,
                Cursor,
            },
            traits::{
                DrawAt,
                DrawError,
                Size,
            },
        },
    },
    graphics,
};

use super::{
    TextPanel,
    text,
};


pub struct AddCommand{
    text_panel: TextPanel,
    cursor:     Cursor,
}

impl AddCommand {
    pub fn create(
        res:   &mut DrawResources,
        input: &Input,
    )
        -> Result<Self, text::CreateError>
    {
        let text_panel = TextPanel::create(res, format!("Add command"))?;

        Ok(
            Self {
                text_panel,
                cursor: input.cursor,
            }
        )
    }
}

impl DrawAt for AddCommand {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>
    {
        let rect = graphics::Rect::new(pos, self.size());

        self.text_panel.panel_color([0.1, 0.0, 0.0, 0.95]); // default color
        if let Some(cursor) = self.cursor {
            if rect.contains(cursor) {
                self.text_panel.panel_color([0.5, 0.0, 0.0, 0.95]);
            }
        }

        self.text_panel.draw_at(res, frame, pos)
    }
}

impl Size for AddCommand {
    fn size(&self) -> graphics::Size {
        self.text_panel.size()
    }
}
