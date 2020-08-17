use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            DrawAt,
            Size,
        },
    },
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


pub struct AddCommand{
    text_panel: TextPanel,
}

impl AddCommand {
    pub fn new(
        res: &mut DrawResources,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut text_panel = TextPanel::new(res, format!("Add command"))?;
        text_panel.panel_color([0.1, 0.0, 0.0, 0.95]);

        Ok(
            Self {
                text_panel
            }
        )
    }
}

impl DrawAt for AddCommand {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.text_panel.draw_at(res, frame, pos)
    }
}

impl Size for AddCommand {
    fn size(&self) -> graphics::Size {
        self.text_panel.size()
    }
}
