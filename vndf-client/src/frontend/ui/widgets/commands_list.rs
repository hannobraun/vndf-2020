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
    Column,
    TextPanel,
    text,
};


#[derive(DrawAt, ProcessInputAt, Size)]
pub struct CommandsList(Column);

impl CommandsList {
    pub fn create(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
    )
        -> Result<Self, text::CreateError>
    {
        let title = TextPanel::create(
            res,
            format!(
                "Commands",
            ),
        )?;

        let mut column = Column::create(margin);
        column.add(title);

        Ok(
            Self(column)
        )
    }
}
