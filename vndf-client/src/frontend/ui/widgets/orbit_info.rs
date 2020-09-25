use vndf_macros::Draw;

use crate::{
    frontend::{
        drawers::DrawResources,
        ui::widgets::Canvas,
    },
    game::Game,
    graphics::{
        self,
        screen::Screen,
    },
    shared::world::{
        self,
        behavior::orbits::Orbit,
    },
};

use super::{
    TextPanel,
    text,
};


#[derive(Draw)]
pub struct OrbitInfo(Canvas);

impl OrbitInfo {
    pub fn create(
        res:    &mut DrawResources,
        orbit:  &Orbit,
        game:   &Game,
        screen: &Screen,
    )
        -> Result<Option<Self>, text::CreateError>
    {
        if let Some((peri_text, peri_pos, apo_text, apo_pos)) =
            Self::text_and_pos(orbit, game, screen)
        {
            let mut canvas = Canvas::create();

            canvas.add_at(
                Box::new(
                    TextPanel::create(
                        res,
                        peri_text,
                    )?,
                ),
                peri_pos,
            );
            canvas.add_at(
                Box::new(
                    TextPanel::create(
                        res,
                        apo_text,
                    )?,
                ),
                apo_pos,
            );

            return Ok(
                Some(
                    Self(canvas)
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

        let pericenter_text = Self::text(
            "Periapsis",
            periapsis_km,
            periapsis_above_surface_km,
        );
        let apocenter_text = Self::text(
            "Apoapsis",
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

        Some((
            pericenter_text,
            pericenter_pos,
            apocenter_text,
            apocenter_pos,
        ))
    }

    fn text(
        name:             &str,
        from_center_km:   world::Length,
        above_surface_km: world::Length,
    )
        -> String
    {
        format!(
            "{}:\n\
            from center: {:.0} km\n\
            above surface: {:.0} km",
            name,
            from_center_km.0,
            above_surface_km.0,
        )
    }
}
