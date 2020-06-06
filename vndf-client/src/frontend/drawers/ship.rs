use crate::{
    frontend::shaders::{
        frag,
        vert,
    },
    game::Game,
    graphics::elements::ScreenElement,
    shared::world::behavior::ships::Ship,
};

use super::{
    DrawResources,
    Frame,
};


pub fn draw_ship(
    res:   &mut DrawResources,
    frame: &mut Frame,
    ship:  &Ship,
    game:  &Game,
)
    -> Option<()>
{
    let transform = ScreenElement::from_ship(ship, game, &frame.screen)?
        .transform(frame.screen.logical_size());

    res.drawables.ship.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::simple::Uniforms {
            color: ship.color.into(),
        },
    );

    Some(())
}
