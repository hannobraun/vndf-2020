use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{frontend::drawers::DrawResources, graphics};

use super::{text, TextPanel};

#[derive(DrawAt, ProcessInputAt, Size)]
pub struct FtlTime(TextPanel);

impl FtlTime {
    pub fn create(
        res: &mut DrawResources,
        jump_time_min: u32,
    ) -> Result<Self, text::CreateError> {
        let text_panel = TextPanel::create(
            res,
            format!("Jump Time (minutes): {}", jump_time_min),
        )?;

        Ok(Self(text_panel))
    }
}
