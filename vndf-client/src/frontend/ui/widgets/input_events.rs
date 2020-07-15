use std::fmt::Write as _;

use vndf_macros::DrawAt;

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
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


#[derive(DrawAt)]
pub struct InputEvents(TextPanel);

impl InputEvents {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let mut text = String::new();
        write!(text, "Input:\n")?;
        for event in game.events.iter().rev() {
            write!(text, "{}\n", event)?;
        }

        let text_panel = TextPanel::new(
            res,
            text,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}

impl Size for InputEvents {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}
