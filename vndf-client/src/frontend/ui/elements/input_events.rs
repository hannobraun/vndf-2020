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


pub struct InputEvents<'r>(TextPanel<'r>);

impl<'r> InputEvents<'r> {
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

        write!(buf, "Input:\n")?;
        for event in game.events.iter().rev() {
            write!(buf, "{}\n", event)?;
        }

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

impl<'r> Element for InputEvents<'r> {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }

    fn draw(self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
