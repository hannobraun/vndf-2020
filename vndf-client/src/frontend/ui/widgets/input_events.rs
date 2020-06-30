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


pub struct InputEvents<'r>(TextPanel<'r>);

impl InputEvents<'_> {
    pub fn new(
        res:  &mut DrawResources,
        buf:  String,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut buf = buf;
        write!(buf, "Input:\n")?;
        for event in game.events.iter().rev() {
            write!(buf, "{}\n", event)?;
        }

        let text_panel = TextPanel::new(
            res,
            buf,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for InputEvents<'_> {
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
