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
pub struct ComponentStats(TextPanel);

impl ComponentStats {
    pub fn create(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Option<Self>, text::CreateError>
    {
        if let Some(diagnostics) = game.state.diagnostics {
            let text_panel = TextPanel::create(
                res,
                format!(
                    "Components:\n\
                    Bodies: {}/{}\n\
                    Crafts: {}/{}\n\
                    Explosions: {}/{}\n\
                    Fuels: {}/{}\n\
                    Healths: {}/{}\n\
                    Planets: {}/{}\n\
                    Players: {}/-\n\
                    Positions: {}/{}\n\
                    Ships: {}/{}\n\
                    Velocities: {}/{}",
                    diagnostics.bodies, game.state.data.bodies.len(),
                    diagnostics.crafts, game.state.data.crafts.len(),
                    diagnostics.explosions, game.state.data.explosions.len(),
                    diagnostics.fuels, game.state.data.fuels.len(),
                    diagnostics.healths, game.state.data.healths.len(),
                    diagnostics.planets, game.state.data.planets.len(),
                    diagnostics.players,
                    diagnostics.positions, game.state.data.positions.len(),
                    diagnostics.ships, game.state.data.ships.len(),
                    diagnostics.velocities, game.state.data.velocities.len(),
                ),
            )?;

            return Ok(
                Some(
                    Self(text_panel)
                )
            );
        }

        Ok(None)
    }
}
