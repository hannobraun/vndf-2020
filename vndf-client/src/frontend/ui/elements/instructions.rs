use std::fmt::{
    self,
    Write as _,
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
    Element,
    TextPanel,
    text::NoBoundsError,
};


pub struct Instructions<'r>(TextPanel<'r>);

impl<'r> Instructions<'r> {
    pub fn new(
        res:  &mut DrawResources,
        buf:  &'r mut String,
        game: &Game,
    )
        -> Result<Self, Error>
    {
        write!(
            buf,
            "Instructions:\n\
            Turn left - {}\n\
            Turn right - {}\n\
            Thrust On - {}\n\
            Thrust Off - {}\n\
            Zoom Camera - Mouse Wheel\n\
            End game - {}",
            game.input.config.input.left,
            game.input.config.input.right,
            game.input.config.input.thrust_on,
            game.input.config.input.thrust_off,
            game.input.config.input.quit,
        )
        .map_err(|err| Error::Fmt(err))?;

        let text_panel = TextPanel::new(
            res,
            buf,
        )
        .map_err(|err| Error::NoBounds(err))?;

        Ok(
            Self(text_panel)
        )
    }
}

impl<'r> Element for Instructions<'r> {
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


#[derive(Debug)]
pub enum Error {
    Fmt(fmt::Error),
    NoBounds(NoBoundsError),
}
