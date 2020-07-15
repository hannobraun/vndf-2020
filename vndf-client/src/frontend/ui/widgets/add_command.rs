use vndf_macros::DrawAt;

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
    TextPanel,
    TextPanelRelatedError,
};


#[derive(DrawAt)]
pub struct AddCommand(TextPanel);

impl AddCommand {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(
            res,
            format!(
                "Add command",
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for AddCommand {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}
