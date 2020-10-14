use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{frontend::drawers::DrawResources, game::Game, graphics};

use super::{text, AddCommand, Column, CommandsList};

#[derive(DrawAt, ProcessInputAt, Size)]
pub struct Commands(Column);

impl Commands {
    pub fn create(
        res: &mut DrawResources,
        margin: graphics::Scalar,
        game: &Game,
    ) -> Result<Self, text::CreateError> {
        let mut column = Column::create(margin);

        column.add(CommandsList::create(res, margin, game)?);
        column.add(AddCommand::create(res)?);

        Ok(Self(column))
    }
}
