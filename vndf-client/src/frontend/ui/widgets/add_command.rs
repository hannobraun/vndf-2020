use vndf_macros::{
    DrawAt,
    Size,
};

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Size,
        },
    },
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


#[derive(DrawAt, Size)]
pub struct AddCommand(TextPanel);

impl AddCommand {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        Ok(
            Self(
                TextPanel::new(res, format!("Add command"))?
                    .panel_color([0.1, 0.0, 0.0, 0.95])
            )
        )
    }
}
