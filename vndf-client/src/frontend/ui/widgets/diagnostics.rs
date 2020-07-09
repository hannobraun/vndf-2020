use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            Draw,
            Widget,
        },
    },
    game::Game,
    graphics,
};

use super::{
    ComponentStats,
    FrameTime,
    InputEvents,
    NetworkStats,
    ScaleFactor,
    Stack,
    TextPanelRelatedError,
};


pub struct Diagnostics(Stack);

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

        let mut diagnostics = Stack::new(margin);

        diagnostics.add(frame_time);
        diagnostics.add(scale_factor);
        diagnostics.add_iter(component_stats);
        diagnostics.add(network_stats);
        diagnostics.add(input_events);

        Ok(
            Self(diagnostics)
        )
    }
}

impl Widget for Diagnostics {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl Draw for Diagnostics {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
