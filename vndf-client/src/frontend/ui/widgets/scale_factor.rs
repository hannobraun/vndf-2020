use vndf_macros::DrawAt;

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


#[derive(DrawAt)]
pub struct ScaleFactor(TextPanel);

impl ScaleFactor {
    pub fn new(
        res:   &mut DrawResources,
        frame: &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
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

impl Size for ScaleFactor {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}
