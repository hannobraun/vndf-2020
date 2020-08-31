use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::Input,
            traits::{
                DrawAt,
                DrawError,
                ProcessInputAt,
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
}

impl AddCommand {
    pub fn create(
        res: &mut DrawResources,
    )
        -> Result<Self, text::CreateError>
    {
        let mut text_panel = TextPanel::create(res, format!("Add command"))?;
        text_panel.panel_color([0.1, 0.0, 0.0, 0.95]); // default color

        Ok(
            Self {
                text_panel,
            }
        )
    }
}

impl ProcessInputAt for AddCommand {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        let rect = graphics::Rect::new(pos, self.size());

        if let Some(cursor) = input.cursor {
            if rect.contains(cursor) {
                self.text_panel.panel_color([0.5, 0.0, 0.0, 0.95]);
            }
        }
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
        self.text_panel.draw_at(res, frame, pos)
    }
}

impl Size for AddCommand {
    fn size(&self) -> graphics::Size {
        self.text_panel.size()
    }
}
