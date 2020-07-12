use crate::{
    game::Game,
    graphics::{
        self,
        screen::Screen,
    },
};


pub struct Elements {
    pub orbit_info: Vec<Element>,
}

impl Elements {
    pub fn new(game: &Game, screen: &Screen) -> Self {
        Self {
            orbit_info: Element::orbit_info(game, screen),
        }
    }
}


pub struct Element {
    pub text: String,
    pub pos:  graphics::Pnt2,
}

impl Element {
    pub fn orbit_info<'r>(game: &'r Game, screen: &'r Screen) -> Vec<Self> {
        game.state.active_orbits()
            .into_iter()
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
