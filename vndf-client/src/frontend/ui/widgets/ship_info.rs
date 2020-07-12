use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::{
            Draw,
            DrawAt,
        },
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
    TextPanel,
    TextPanelRelatedError,
};


pub struct ShipInfo {
    text_panel: TextPanel,
    pos:        graphics::Pnt2,
}

impl ShipInfo {
    pub fn new(
        res:    &mut DrawResources,
        ship:   &Ship,
        game:   &Game,
        screen: &Screen,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        if let Some((text, pos)) = Self::text_and_pos(ship, game, screen) {
            let text_panel = TextPanel::new(
                res,
                text,
            )?;

            return Ok(
                Some(
                    Self {
                        text_panel,
                        pos,
                    }
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

impl Draw for ShipInfo {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    ) {
        self.text_panel.draw_at(res, frame, self.pos)
    }
}
