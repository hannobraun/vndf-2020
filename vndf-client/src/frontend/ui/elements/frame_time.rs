use std::fmt::Write as _;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements,
    },
    game::Game,
    graphics,
};

use super::{
    Element,
    TextPanel,
    TextPanelRelatedError,
};


pub struct FrameTime<'r>(TextPanel<'r>);

impl<'r> FrameTime<'r> {
    pub fn new(
        res:  &mut DrawResources,
        buf:  &'r mut String,
        game: &Game,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        if !game.input.config.diagnostics {
            return Ok(None);
        }

        let report = game.state.frame_time.report();
        write!(
            buf,
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        )?;

        let text_panel = TextPanel::new(
            res,
            buf,
        )?;

        Ok(
            Some(
                Self(text_panel)
            )
        )
    }
}

impl<'r> elements::Size for FrameTime<'r> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl<'r> Element for FrameTime<'r> {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
