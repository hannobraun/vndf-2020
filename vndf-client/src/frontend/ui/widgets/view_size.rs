use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            Draw,
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


pub struct ViewSize(TextPanel);

impl ViewSize {
    pub fn new(
        res:   &mut DrawResources,
        frame: &Frame,
        game:  &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let size = game.state.camera.world_size_on_screen(&frame.screen);

        let width_km  = size.width  / 1000.0;
        let height_km = size.height / 1000.0;

        let text_panel = TextPanel::new(
            res,
            format!(
                "View Size (km):\n\
                {:.0} x {:.0}",
                width_km,
                height_km,
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for ViewSize {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl Draw for ViewSize {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
