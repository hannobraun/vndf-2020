use std::fmt::Write as _;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::Widget,
    },
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct ViewSize<'r>(TextPanel<'r>);

impl<'r> ViewSize<'r> {
    pub fn new(
        res:   &mut DrawResources,
        frame: &Frame,
        buf:   &'r mut String,
        game:  &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let size = game.state.camera.world_size_on_screen(&frame.screen);

        let width_km  = size.width  / 1000.0;
        let height_km = size.height / 1000.0;

        write!(
            buf,
            "View Size (km):\n\
            {:.0} x {:.0}",
            width_km,
            height_km,
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

impl Widget for ViewSize<'_> {
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
