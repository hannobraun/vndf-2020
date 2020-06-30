use std::fmt::Write as _;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::Widget,
    },
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct FrameTime<'r>(TextPanel<'r>);

impl FrameTime<'_> {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let report = game.state.frame_time.report();
        let mut text = String::new();
        write!(
            text,
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        )?;

        let text_panel = TextPanel::new(
            res,
            text,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for FrameTime<'_> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
