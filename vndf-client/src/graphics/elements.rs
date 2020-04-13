use crate::{
    game::Game,
    graphics::{
        self,
        transforms,
    },
    shared::world::{
        self,
        behavior::{
            ships::Ship,
            planets::Planet,
        },
    },
};


pub struct UiElement {
    pub size:  graphics::Size,
    pub pos:   graphics::Pnt2,
    pub angle: graphics::Angle,
}

impl UiElement {
    pub fn from_ship(ship: &Ship, game: &Game, screen_size: graphics::Size)
        -> Option<Self>
    {
        let craft = game.state.data.crafts.get(&ship.craft)?;
        let body  = game.state.data.bodies.get(&craft.body)?;
        let pos   = game.state.data.positions.get(&body.pos)?;

        let pos = transforms::world_to_screen(&game.state.camera, screen_size)
            .transform_point(pos.0);
        let angle = body.dir.angle_from_x_axis();

        Some(
            Self {
                size: graphics::Size::new(30.0, 30.0),
                pos,
                angle,
            }
        )
    }
}


pub struct WorldElement {
    pub size:  world::Size,
    pub pos:   world::Pnt2,
    pub angle: world::Angle,
}

impl From<&Planet> for WorldElement {
    fn from(planet: &Planet) -> Self {
        Self {
            size:  world::Size::from_lengths(planet.size, planet.size),
            pos:   planet.pos,
            angle: world::Angle::zero(),
        }
    }
}
