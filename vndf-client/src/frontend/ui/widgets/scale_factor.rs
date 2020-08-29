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
pub struct ScaleFactor(TextPanel);

impl ScaleFactor {
    pub fn new(
        res:   &mut DrawResources,
        frame: &Frame,
    )
        -> Result<Self, text::CreateError>
    {
        let text_panel = TextPanel::new(
            res,
            format!(
                "Scale factor: {}",
                frame.screen.scale_factor,
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
