use vndf_macros::{
    DrawAt,
    ProcessInputAt,
    Size,
};

use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    text,
};


#[derive(DrawAt, ProcessInputAt, Size)]
pub struct ViewSize(TextPanel);

impl ViewSize {
    pub fn create(
        res:   &mut DrawResources,
        frame: &Frame,
        game:  &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let size = game.state.camera.world_size_on_screen(&frame.screen);

        let width_km  = size.width  / 1000.0;
        let height_km = size.height / 1000.0;

        let text_panel = TextPanel::create(
            res,
            format!(
                "View Size (km):\n\
                {:.0} x {:.0}",
                width_km,
                height_km,
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
