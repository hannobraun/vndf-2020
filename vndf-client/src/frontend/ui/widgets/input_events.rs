use std::fmt::Write as _;

use vndf_macros::{
    DrawAt,
    Size,
};

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
    text,
};


#[derive(DrawAt, Size)]
pub struct InputEvents(TextPanel);

impl InputEvents {
    pub fn create(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let mut text = String::new();
        write!(text, "Input:\n")?;
        for event in game.events.iter().rev() {
            write!(text, "{}\n", event)?;
        }

        let text_panel = TextPanel::create(
            res,
            text,
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
