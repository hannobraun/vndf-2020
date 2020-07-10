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
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct ScaleFactor(TextPanel);

impl ScaleFactor {
    pub fn new(
        res:   &mut DrawResources,
        frame: &Frame,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(
            res,
            format!(
                "Scale factor: {}",
                frame.screen.scale_factor,
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Widget for ScaleFactor {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}

impl DrawAt for ScaleFactor {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.0.draw(res, frame, pos)
    }
}
