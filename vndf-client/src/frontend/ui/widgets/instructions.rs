use vndf_macros::DrawAt;

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
    game::Game,
    graphics,
};

use super::{
    TextPanel,
    TextPanelRelatedError,
};


#[derive(DrawAt)]
pub struct Instructions(TextPanel);

impl Instructions {
    pub fn new(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Self, TextPanelRelatedError>
    {
        let text_panel = TextPanel::new(
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

impl Widget for Instructions {
    fn size(&self) -> graphics::Size {
        self.0.size()
    }
}
