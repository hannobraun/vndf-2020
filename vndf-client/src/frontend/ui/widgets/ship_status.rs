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
    shared::world::behavior::{
        crafts::Fuel,
        health::Health,
    }
};

use super::{
    TextPanel,
    text
};


#[derive(DrawAt, Size)]
pub struct ShipStatus(TextPanel);

impl ShipStatus {
    pub fn create(
        res:  &mut DrawResources,
        game: &Game,
    )
        -> Result<Option<Self>, text::CreateError>
    {
        fn components(game: &Game) -> Option<(&Fuel, &Health)> {
            let ship   = game.state.own_ship()?;
            let craft  = game.state.data.crafts.get(&ship.craft)?;
            let fuel   = game.state.data.fuels.get(&craft.fuel)?;
            let health = game.state.data.healths.get(&craft.health)?;

            Some((fuel, health))
        }

        if let Some((fuel, health)) = components(game) {
            let text_panel = TextPanel::create(
                res,
                format!(
                    "Ship Status\n\
                    Structural Integrity: {:.2}\n\
                    Fuel: {:.2}",
                    health.value,
                    fuel.0,
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
