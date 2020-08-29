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
    text,
};


#[derive(DrawAt, Size)]
pub struct CommandsList(TextPanel);

impl CommandsList {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, text::CreateError>
    {
        let text_panel = TextPanel::create(
            res,
            format!(
                "Commands",
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
