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


pub struct ComponentStats<'r>(TextPanel<'r>);

impl<'r> ComponentStats<'r> {
    pub fn new(
        res:  &mut DrawResources,
        buf:  String,
        game: &Game,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        if let Some(diagnostics) = game.state.diagnostics {
            let mut buf = buf;
            write!(
                buf,
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
            )?;

            let text_panel = TextPanel::new(
                res,
                buf,
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

impl Widget for ComponentStats<'_> {
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
