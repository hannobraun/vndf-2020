use crate::{
    game::Game,
    graphics::{
        self,
        elements::ScreenElement,
        screen::Screen,
    },
};


pub struct Elements {
    pub instructions:    Element,
    pub frame_time:      Option<Element>,
    pub diagnostics:     Option<Element>,
    pub input_events:    Option<Element>,
    pub own_ship_status: Option<Element>,

    pub ship_info:  Vec<Element>,
    pub orbit_info: Vec<Element>,
}

impl Elements {
    pub fn new(game: &Game, screen: &Screen) -> Self {
        Self {
            instructions: Element::instructions(game),
            frame_time:   Element::frame_time(game),
            diagnostics:  Element::diagnostics(game),
            input_events: Element::input_events(game),
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
    pub fn instructions(game: &Game) -> Self {
        let text = format!(
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
        );

        let pos = graphics::Pnt2::new(20.0, 20.0);

        Self {
            text,
            pos,
        }
    }

    pub fn frame_time(game: &Game) -> Option<Self> {
        if !game.input.config.diagnostics {
            return None;
        }

        let report = game.state.frame_time.report();
        let text = format!(
            "Frame time:\n{} ms (avg {}/{}/{})",
            report.latest.whole_milliseconds(),
            report.avg_1.whole_milliseconds(),
            report.avg_2.whole_milliseconds(),
            report.avg_3.whole_milliseconds(),
        );

        let pos = graphics::Pnt2::new(20.0, 180.0);

        Some(
            Self {
                text,
                pos,
            }
        )
    }

    pub fn diagnostics(game: &Game) -> Option<Self> {
        if !game.input.config.diagnostics {
            return None;
        }

        game.state.diagnostics.map(|diagnostics| {
            let text = format!(
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
                Velocities: {}/{}\n\
                ---\n\
                Updates per s: {}\n\
                Removals per s: {}",
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
                game.state.statistics.updates.len(),
                game.state.statistics.removals.len(),
            );

            let pos = graphics::Pnt2::new(20.0, 220.0);

            Self {
                text,
                pos,
            }
        })
    }

    pub fn input_events(game: &Game) -> Option<Self> {
        if !game.input.config.diagnostics {
            return None;
        }

        let mut text = String::from("Input:\n");
        for event in game.events.iter().rev() {
            text.push_str(&format!("{}\n", event));
        }

        let pos = graphics::Pnt2::new(20.0, 520.0);

        Some(
            Self {
                text,
                pos,
            }
        )
    }

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
