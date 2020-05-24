use super::{
    DrawResources,
    Frame,
};

use crate::{
    frontend::shaders::{
        frag,
        vert,
    },
    game::Game,
    graphics::elements::ScreenElement,
    shared::world::behavior::explosions::Explosion,
};


pub fn draw_explosion(
    res:       &DrawResources,
    frame:     &mut Frame,
    explosion: &Explosion,
    game:      &Game,
)
    -> Option<()>
{
    let transform =
        ScreenElement::from_explosion(
            explosion,
            game,
            &frame.screen,
        )?
        .transform(frame.screen.size);

    res.drawables.explosion.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::explosion::Uniforms {
            strength_total: explosion.strength_total,
            strength_left:  explosion.strength_left,
        },
    );

    Some(())
}
