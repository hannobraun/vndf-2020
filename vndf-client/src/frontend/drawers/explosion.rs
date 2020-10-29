use crate::{
    frontend::{
        shaders::{frag, vert},
        uniforms,
    },
    game::Game,
    graphics::elements::ScreenElement,
    shared::world::features::explosions::Explosion,
};

use super::{DrawResources, Frame};

pub fn draw_explosion(
    res: &mut DrawResources,
    frame: &mut Frame,
    explosion: &Explosion,
    game: &Game,
) -> Option<()> {
    let transform =
        ScreenElement::from_explosion(explosion, game, &frame.screen)?
            .transform(&frame.screen);

    res.drawables.explosion.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::explosion::Uniforms {
            strength_total: explosion.strength_total as uniforms::Float,
            strength_left: explosion.strength_left as uniforms::Float,
        },
    );

    Some(())
}
