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
    TextPanel,
    TextPanelRelatedError,
};


#[derive(DrawAt, Size)]
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
