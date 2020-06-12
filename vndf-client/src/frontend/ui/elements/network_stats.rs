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
    TextPanel,
    TextPanelRelatedError,
};


pub struct NetworkStats<'r>(TextPanel<'r>);

impl<'r> NetworkStats<'r> {
    pub fn new(
        res:  &mut DrawResources,
        buf:  &'r mut String,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        write!(
            buf,
            "Network:\n\
            Updates per s: {}\n\
            Removals per s: {}",
            game.state.statistics.updates.len(),
            game.state.statistics.removals.len(),
        )?;

        let text_panel = TextPanel::new(
            res,
            buf,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl<'r> elements::Size for NetworkStats<'r> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl<'r> elements::Draw for NetworkStats<'r> {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
