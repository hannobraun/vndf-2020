use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{
    frontend::{drawers::DrawResources, ui::input::Action},
    graphics,
};

use super::{text, Button};

#[derive(DrawAt, Size, ProcessInputAt)]
pub struct AddCommand(Button);

impl AddCommand {
    pub fn create(res: &mut DrawResources) -> Result<Self, text::CreateError> {
        let button = Button::create(
            res,
            format!("Add command"),
            Action::AddCommand,
            [0.1, 0.0, 0.0, 0.95],
            [0.5, 0.0, 0.0, 0.95],
        )?;

        Ok(Self(button))
    }
}
