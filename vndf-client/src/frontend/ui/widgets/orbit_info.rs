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
        screen::Screen,
    },
    shared::world::behavior::orbits::Orbit,
};

use super::{
    Positioned,
    TextPanel,
    TextPanelRelatedError,
};


#[derive(Draw)]
pub struct OrbitInfo {
    periapsis: Positioned<TextPanel>,
    apoapsis:  Positioned<TextPanel>,
}

impl OrbitInfo {
    pub fn new(
        res:    &mut DrawResources,
        orbit:  &Orbit,
        game:   &Game,
        screen: &Screen,
    )
        -> Result<Option<Self>, TextPanelRelatedError>
    {
        if let Some((peri_text, peri_pos, apo_text, apo_pos)) =
            Self::text_and_pos(orbit, game, screen)
        {
            let periapsis = Positioned {
                widget: TextPanel::new(
                    res,
                    peri_text,
                )?,
                position: peri_pos,
            };
            let apoapsis = Positioned {
                widget: TextPanel::new(
                    res,
                    apo_text,
                )?,
                position: apo_pos,
            };

            return Ok(
                Some(
                    Self {
                        periapsis,
                        apoapsis,
                    }
                )
            );
        }

        Ok(None)
    }

    fn text_and_pos(
        orbit:  &Orbit,
        game:   &Game,
        screen: &Screen,
    )
        -> Option<(String, graphics::Pnt2, String, graphics::Pnt2)>
    {
        // If our orbit is nearly circular, the computed apses will jump around
        // like crazy. Let's make sure we have a minimum eccentricity, so they
        // become well-defined.
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
            periapsis_km.0,
            periapsis_above_surface_km.0,
        );
        let apocenter_text = format!(
            "Apoapsis:\n\
            from center: {:.0} km\n\
            above surface:{:.0} km",
            apoapsis_km.0,
            apoapsis_above_surface_km.0,
        );

        let pericenter_pos = game.state.camera.world_to_screen(
            &screen,
            orbit.periapsis.position,
        );
        let apocenter_pos = game.state.camera.world_to_screen(
            &screen,
            orbit.apoapsis.position,
        );

        Some((
            pericenter_text,
            pericenter_pos,
            apocenter_text,
            apocenter_pos,
        ))
    }
}
