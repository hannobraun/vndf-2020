use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
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

impl Widget for NetworkStats {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl DrawAt for NetworkStats {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw_at(res, frame, pos)
    }
}
