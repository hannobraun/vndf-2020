use std::fmt::Write as _;

use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    game::Game,
    graphics,
};

use super::{
    Element,
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
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        if !game.input.config.diagnostics {
            return Ok(None);
        }

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
            Some(
                Self(text_panel)
            )
        )
    }
}

impl<'r> Element for NetworkStats<'r> {
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
