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
pub struct Instructions(TextPanel);

impl Instructions {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, text::CreateError>
    {
        let text_panel = TextPanel::create(
            res,
            format!(
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
            ),
        )?;

        Ok(
            Self(text_panel)
        )
    }
}
