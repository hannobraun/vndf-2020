use vndf_macros::Draw;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::Draw,
    },
    game::Game,
    graphics::{
        self,
        elements::ScreenElement,
        screen::Screen,
    },
    shared::world::behavior::ships::Ship,
};

use super::{
    Positioned,
    TextPanel,
    text,
};


#[derive(Draw)]
pub struct ShipInfo(Positioned<TextPanel>);

impl ShipInfo {
    pub fn create(
        res:    &mut DrawResources,
        ship:   &Ship,
        game:   &Game,
        screen: &Screen,
    )
        -> Result<Option<Self>, text::CreateError>
    {
        if let Some((text, pos)) = Self::text_and_pos(ship, game, screen) {
            let text_panel = TextPanel::create(
                res,
                text,
            )?;

            return Ok(
                Some(
                    Self(
                        Positioned {
                            widget:   text_panel,
                            position: pos,
                        }
                    )
                )
            );
        }

        Ok(None)
    }

    fn text_and_pos(
        ship:   &Ship,
        game:   &Game,
        screen: &Screen,
    )
        -> Option<(String, graphics::Pnt2)>
    {
        let craft = game.state.data.crafts.get(&ship.craft)?;
        let body  = game.state.data.bodies.get(&craft.body)?;
        let pos   = game.state.data.positions.get(&body.pos)?;
        let vel   = game.state.data.velocities.get(&body.vel)?;

        let pos_km = pos.0 / 1000.0;
        let vel_km = vel.0 / 1000.0;

        let text = format!(
            "Pos: {:.0}/{:.0}\n\
            Vel: {:.0}/{:.0} ({:.0})",
            pos_km.x, pos_km.y,
            vel_km.x, vel_km.y, vel_km.length(),
        );

        let element = ScreenElement::from_ship(
            ship,
            game,
            screen,
        )?;
        let pos = element.pos + graphics::Vec2::new(20.0, -20.0);

        Some((text, pos))
    }
}
