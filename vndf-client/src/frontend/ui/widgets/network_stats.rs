use vndf_macros::{DrawAt, ProcessInputAt, Size};

use crate::{frontend::drawers::DrawResources, game::Game, graphics};

use super::{text, TextPanel};

#[derive(DrawAt, ProcessInputAt, Size)]
pub struct NetworkStats(TextPanel);

impl NetworkStats {
    pub fn create(
        res: &mut DrawResources,
        game: &Game,
    ) -> Result<Self, text::CreateError> {
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

        Ok(Self(text_panel))
    }
}
