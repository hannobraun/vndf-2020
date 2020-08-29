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
    game::Game,
    graphics,
};

use super::{
    Column,
    ComponentStats,
    FrameTime,
    InputEvents,
    NetworkStats,
    ScaleFactor,
    text,
};


#[derive(DrawAt, Size)]
pub struct Diagnostics(Column);

impl Diagnostics {
    pub fn create(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        game:   &Game,
        frame:  &Frame,
    )
        -> Result<Self, text::CreateError>
    {
        let frame_time = FrameTime::create(
            res,
            game,
        )?;
        let scale_factor = ScaleFactor::create(
            res,
            frame,
        )?;
        let component_stats = ComponentStats::create(
            res,
            game,
        )?;
        let network_stats = NetworkStats::create(
            res,
            game,
        )?;
        let input_events = InputEvents::new(
            res,
            game,
        )?;

        let mut column = Column::new(margin);

        column.add(frame_time);
        column.add(scale_factor);
        column.add_iter(component_stats);
        column.add(network_stats);
        column.add(input_events);

        Ok(
            Self(column)
        )
    }
}
