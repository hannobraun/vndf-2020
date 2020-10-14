use vndf_macros::{DrawAt, Size};

use crate::{
    frontend::{
        drawers::DrawResources,
        ui::{
            input::{Action, Input},
            traits::{ProcessInputAt, Size},
        },
    },
    graphics,
};

use super::{text, TextPanel};

#[derive(DrawAt, Size)]
pub struct AddCommand(TextPanel);

impl AddCommand {
    pub fn create(res: &mut DrawResources) -> Result<Self, text::CreateError> {
        let mut text_panel = TextPanel::create(res, format!("Add command"))?;
        text_panel.panel_color([0.1, 0.0, 0.0, 0.95]); // default color

        Ok(Self(text_panel))
    }
}

impl ProcessInputAt for AddCommand {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        let rect = graphics::Rect::new(pos, self.size());

        if let Some(cursor) = input.cursor {
            if rect.contains(cursor) {
                self.0.panel_color([0.5, 0.0, 0.0, 0.95]);

                if input.click {
                    input.actions.push(Action::AddCommand);
                }
            }
        }
    }
}
