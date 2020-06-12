use crate::{
    game::Game,
    graphics::{
        self,
        elements::ScreenElement,
        screen::Screen,
    },
};


pub struct Elements {
    pub own_ship_status: Option<Element>,

    pub ship_info:  Vec<Element>,
    pub orbit_info: Vec<Element>,
}

impl Elements {
    pub fn new(game: &Game, screen: &Screen) -> Self {
        Self {
            own_ship_status: Element::own_ship_status(game, screen),

            orbit_info: Element::orbit_info(game, screen),
            ship_info:  Element::ship_info(game, screen),
        }
    }
}


pub struct Element {
    pub text: String,
    pub pos:  graphics::Pnt2,
}

impl Element {
    pub fn own_ship_status(game: &Game, screen: &Screen) -> Option<Self> {
        let ship   = game.state.own_ship()?;
        let craft  = game.state.data.crafts.get(&ship.craft)?;
        let fuel   = game.state.data.fuels.get(&craft.fuel)?;
        let health = game.state.data.healths.get(&craft.health)?;

        let text = format!(
            "Ship Status\n\
            Structural Integrity: {:.2}\n\
            Fuel: {:.2}",
            health.value,
            fuel.0,
        );

        let width = screen.logical_size().width;
        let pos = graphics::Pnt2::new(width - 200.0, 20.0);

        Some(
            Self {
                text,
                pos,
            }
        )
    }

    pub fn ship_info<'r>(game: &'r Game, screen: &'r Screen) -> Vec<Self> {
        game.state.data.ships.values()
            .filter_map(move |ship| {
                let craft = game.state.data.crafts.get(&ship.craft)?;
                let body  = game.state.data.bodies.get(&craft.body)?;
                let pos_w = game.state.data.positions.get(&body.pos)?;
                let vel   = game.state.data.velocities.get(&body.vel)?;

                let pos_km = pos_w.0 / 1000.0;
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

                Some(
                    Self {
                        text,
                        pos,
                    }
                )
            })
            .collect()
    }

    pub fn orbit_info<'r>(game: &'r Game, screen: &'r Screen) -> Vec<Self> {
        game.state.active_orbits()
            .filter_map(move |orbit| {
                // Display periapsis and apoapsis
                //
                // If our orbit is nearly circular, the computed apses will jump
                // around like crazy. Let's make sure we have a minimum of
                // eccentricity, so they become well-defined.
                if orbit.eccentricity.length() <= 0.01 {
                    return None;
                }

                let periapsis_km = orbit.periapsis.distance / 1000.0;
                let apoapsis_km  = orbit.apoapsis.distance  / 1000.0;

                let periapsis_above_surface_km =
                    orbit.periapsis.from_surface / 1000.0;
                let apoapsis_above_surface_km =
                    orbit.apoapsis.from_surface / 1000.0;

                let pericenter_text = format!(
                    "Periapsis:\n\
                    from center: {:.0} km\n\
                    above surface:{:.0} km",
                    periapsis_km,
                    periapsis_above_surface_km,
                );
                let apocenter_text = format!(
                    "Apoapsis:\n\
                    from center: {:.0} km\n\
                    above surface:{:.0} km",
                    apoapsis_km,
                    apoapsis_above_surface_km,
                );

                let pericenter_pos = game.state.camera.world_to_screen(
                    &screen,
                    orbit.periapsis.position,
                );
                let apocenter_pos = game.state.camera.world_to_screen(
                    &screen,
                    orbit.apoapsis.position,
                );

                Some(
                    vec![
                        Self {
                            text: pericenter_text,
                            pos:  pericenter_pos,
                        },
                        Self {
                            text: apocenter_text,
                            pos:  apocenter_pos,
                        },
                    ]
                )
            })
            .flatten()
            .collect()
    }
}
