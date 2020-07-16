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
pub struct NetworkStats(TextPanel);

impl NetworkStats {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(
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
