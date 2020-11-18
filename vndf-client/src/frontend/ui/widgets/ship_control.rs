use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{frontend::drawers::DrawResources, game::Game, graphics};

use super::{text, Column, Commands, FtlJump, FtlTime, ShipStatus};

#[derive(DrawAt, ProcessInputAt, Size)]
pub struct ShipControl(Column);

impl ShipControl {
    pub fn create(
        res: &mut DrawResources,
        margin: graphics::Scalar,
        game: &Game,
        jump_time_min: u32,
    ) -> Result<Option<Self>, text::CreateError> {
        let ship_status = ShipStatus::create(res, game)?;
        let commands = Commands::create(res, margin, game)?;

        let mut column = Column::create(margin);

        if let Some(ship_status) = ship_status {
            column.add(ship_status);
            column.add(commands);
            column.add(FtlTime::create(res, jump_time_min)?);
            column.add(FtlJump::create(res)?);
        }

        Ok(Some(Self(column)))
    }
}
