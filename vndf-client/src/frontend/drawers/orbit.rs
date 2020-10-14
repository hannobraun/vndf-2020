use crate::{
    frontend::{
        shaders::{frag, vert},
        uniforms,
    },
    game::Game,
    graphics::{self, elements::WorldElement},
    shared::world::features::orbits::Orbit,
};

use super::{DrawResources, Frame};

pub fn draw_orbit(
    res: &mut DrawResources,
    frame: &mut Frame,
    orbit: &Orbit,
    game: &Game,
) -> Option<()> {
    let element = WorldElement::from(orbit);

    let transform = element.transform(&game.state.camera, &frame.screen);

    let pixel_per_m = game.state.camera.pixels_per_meter(&frame.screen);
    let pixel_per_u = [
        pixel_per_m * element.size.width as graphics::Scalar,
        pixel_per_m * element.size.height as graphics::Scalar,
    ];
    let u_per_pixel = [1.0 / pixel_per_u[0], 1.0 / pixel_per_u[1]];

    let orbiter_angle_abs = orbit.orbiter.pos.to_vector().angle_from_x_axis();
    let orbiter_angle_to_orbit = (orbiter_angle_abs - orbit.arg_of_periapsis).signed();

    let orbiter_dir = orbit
        .orbiter
        .pos
        .to_vector()
        .angle_to(orbit.orbiter.vel)
        .radians;
    let orbiter_dir = if orbiter_dir < 0.0 {
        -1.0
    } else if orbiter_dir > 0.0 {
        1.0
    } else {
        0.0
    };

    res.drawables.orbit.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::orbit::Uniforms {
            u_per_pixel: u_per_pixel.into(),
            orbiter_angle: orbiter_angle_to_orbit.radians as uniforms::Float,
            orbiter_dir,
            ..frag::orbit::Uniforms::default()
        },
    );

    Some(())
}
