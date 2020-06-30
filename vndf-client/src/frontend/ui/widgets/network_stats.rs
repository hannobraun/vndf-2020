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


pub struct NetworkStats<'r>(TextPanel<'r>);

impl NetworkStats<'_> {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut text = String::new();
        write!(
            text,
            "Network:\n\
            Updates per s: {}\n\
            Removals per s: {}",
            game.state.statistics.updates.len(),
            game.state.statistics.removals.len(),
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

impl Widget for NetworkStats<'_> {
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
