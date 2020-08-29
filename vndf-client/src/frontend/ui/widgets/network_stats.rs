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
    text,
};


#[derive(DrawAt, Size)]
pub struct NetworkStats(TextPanel);

impl NetworkStats {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let text_panel = TextPanel::create(
            res,
            format!(
                "Network:\n\
                Updates per s: {}\n\
                Removals per s: {}",
                game.state.statistics.updates.len(),
                game.state.statistics.removals.len(),
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
