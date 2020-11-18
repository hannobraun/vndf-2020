use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{
    frontend::{drawers::DrawResources, ui::input::Action},
    game::Game,
    graphics,
};

use super::{text, Button, Column, Commands, FtlJump, FtlTime, ShipStatus};

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

        let up = Button::create(
            res,
            format!("+"),
            Action::FtlTimeUp,
            [0.1, 0.0, 0.0, 0.95],
            [0.5, 0.0, 0.0, 0.95],
        )?;
        let down = Button::create(
            res,
            format!("-"),
            Action::FtlTimeDown,
            [0.1, 0.0, 0.0, 0.95],
            [0.5, 0.0, 0.0, 0.95],
        )?;

        let mut column = Column::create(margin);

        if let Some(ship_status) = ship_status {
            column.add(ship_status);
            column.add(commands);
            column.add(FtlTime::create(res, jump_time_min)?);
            column.add(up);
            column.add(down);
            column.add(FtlJump::create(res)?);
        }

        Ok(Some(Self(column)))
    }
}
