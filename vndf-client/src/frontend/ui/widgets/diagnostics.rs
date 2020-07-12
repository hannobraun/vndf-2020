use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Widget,
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
    TextPanelRelatedError,
};


pub struct Diagnostics(Column);

impl Diagnostics {
    pub fn new(
        res:    &mut DrawResources,
        margin: graphics::Scalar,
        game:   &Game,
        frame:  &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let frame_time = FrameTime::new(
            res,
            game,
        )?;
        let scale_factor = ScaleFactor::new(
            res,
            frame,
        )?;
        let component_stats = ComponentStats::new(
            res,
            game,
        )?;
        let network_stats = NetworkStats::new(
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

impl Widget for Diagnostics {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl DrawAt for Diagnostics {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw_at(res, frame, pos)
    }
}
