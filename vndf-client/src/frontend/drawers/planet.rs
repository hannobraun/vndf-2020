use crate::{
    frontend::shaders::{
        frag,
        vert,
    },
    game::Game,
    graphics::elements::WorldElement,
    shared::world::behavior::planets::Planet,
};

use super::{
    DrawResources,
    Frame,
};


pub fn draw_planet(
    res:    &mut DrawResources,
    frame:  &mut Frame,
    planet: &Planet,
    game:   &Game,
) {
    let transform = WorldElement::from(planet)
        .transform(&game.state.camera, frame.screen.size);

    res.drawables.planet.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::planet::Uniforms::default(),
    );
}
