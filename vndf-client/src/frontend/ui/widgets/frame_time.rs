use vndf_macros::{
    DrawAt,
    Size,
};

use crate::{
    frontend::drawers::DrawResources,
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    text,
};


#[derive(DrawAt, Size)]
pub struct FrameTime(TextPanel);

impl FrameTime {
    pub fn create(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let report = game.state.frame_time.report();

        let text_panel = TextPanel::create(
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
