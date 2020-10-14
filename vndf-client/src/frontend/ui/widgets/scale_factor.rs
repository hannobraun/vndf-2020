use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{
    frontend::drawers::{DrawResources, Frame},
    graphics,
};

use super::{text, TextPanel};

#[derive(DrawAt, ProcessInputAt, Size)]
pub struct ScaleFactor(TextPanel);

impl ScaleFactor {
    pub fn create(res: &mut DrawResources, frame: &Frame) -> Result<Self, text::CreateError> {
        let text_panel =
            TextPanel::create(res, format!("Scale factor: {}", frame.screen.scale_factor,))?;

        Ok(Self(text_panel))
    }
}
