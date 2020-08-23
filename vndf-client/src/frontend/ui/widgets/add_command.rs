use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::Pointer,
            widgets::{
                DrawAt,
                Size,
            },
        },
    },
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct AddCommand{
    text_panel: TextPanel,
    pointer:    Pointer,
}

impl AddCommand {
    pub fn new(
        res:     &mut DrawResources,
        pointer: Pointer,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(res, format!("Add command"))?;

        Ok(
            Self {
                text_panel,
                pointer,
            }
        )
    }
}

impl DrawAt for AddCommand {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let rect = graphics::Rect::new(pos, self.size());

        self.text_panel.panel_color([0.1, 0.0, 0.0, 0.95]); // default color
        if let Some(pointer) = self.pointer {
            if rect.contains(pointer) {
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
