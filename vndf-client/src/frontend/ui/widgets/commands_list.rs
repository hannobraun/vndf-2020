use vndf_macros::{
    DrawAt,
    ProcessInputAt,
    Size,
};

use crate::{
    frontend::drawers::DrawResources,
    graphics,
};

use super::{
    TextPanel,
    text,
};


#[derive(DrawAt, ProcessInputAt, Size)]
pub struct CommandsList(TextPanel);

impl CommandsList {
    pub fn create(
        res: &mut DrawResources,
    )
        -> Result<Self, text::CreateError>
    {
        let title = TextPanel::create(
            res,
            format!(
                "Commands",
            ),
        )?;

        Ok(
            Self(title)
        )
    }
}
