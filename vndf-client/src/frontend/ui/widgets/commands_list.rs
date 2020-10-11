use vndf_macros::{
    DrawAt,
    ProcessInputAt,
    Size,
};

use crate::{
    frontend::drawers::DrawResources,
    game::Game,
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
        game:   &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let mut column = Column::create(margin / 3.0);
        column.add(TextPanel::create(res, format!("Commands"))?);
        for command in &game.state.commands {
            column.add(TextPanel::create(res, format!("{}", command))?);
        }

        Ok(
            Self(column)
        )
    }
}
