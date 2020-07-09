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
    TextPanel,
    TextPanelRelatedError,
};


pub struct FrameTime(TextPanel);

impl FrameTime {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let report = game.state.frame_time.report();

        let text_panel = TextPanel::new(
            res,
            format!(
                "Frame time:\n{} ms (avg {}/{}/{})",
                report.latest.whole_milliseconds(),
                report.avg_1.whole_milliseconds(),
                report.avg_2.whole_milliseconds(),
                report.avg_3.whole_milliseconds(),
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for FrameTime {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl Draw for FrameTime {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
