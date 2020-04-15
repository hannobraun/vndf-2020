use crate::{
    game::{
        Game,
        camera::Camera,
    },
    graphics::{
        self,
        transforms,
    },
    shared::world::{
        self,
        behavior::{
            crafts::Craft,
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
    pub fn from_ship(
        ship:   &Ship,
        game:   &Game,
        screen: graphics::Size,
    )
        -> Option<Self>
    {
        let craft = game.state.data.crafts.get(&ship.craft)?;

        Self::from_craft(
            craft,
            graphics::Size::new(30.0, 30.0),
            game,
            screen,
        )
    }

    pub fn from_craft(
        craft:  &Craft,
        size:   graphics::Size,
        game:   &Game,
        screen: graphics::Size,
    )
        -> Option<Self>
    {
        let body = game.state.data.bodies.get(&craft.body)?;
        let pos  = game.state.data.positions.get(&body.pos)?;

        let pos = transforms::world_to_screen(&game.state.camera, screen)
            .transform_point(pos.0);
        let angle = -body.dir.angle_from_x_axis();

        Some(
            Self {
                size,
                pos,
                angle,
            }
        )
    }

    pub fn transform(&self, screen_size: graphics::Size) -> Transform {
        transforms::local_to_screen(self)
            .post_transform(
                &transforms::screen_to_homogeneous(screen_size)
            )
            .to_3d()
            .to_row_arrays()
    }
}

impl Default for UiElement {
    fn default() -> Self {
        Self {
            size:  graphics::Size::new(1.0, 1.0),
            pos:   graphics::Pnt2::origin(),
            angle: graphics::Angle::zero(),
        }
    }
}


pub struct WorldElement {
    pub size:  world::Size,
    pub pos:   world::Pnt2,
    pub angle: world::Angle,
}

impl WorldElement {
    pub fn transform(&self, camera: &Camera, screen_size: graphics::Size)
        -> Transform
    {
        transforms::local_to_world(self)
            .post_transform(
                &transforms::world_to_screen(camera, screen_size)
            )
            .post_transform(
                &transforms::screen_to_homogeneous(screen_size)
            )
            .to_3d()
            .to_row_arrays()
    }
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


pub type Transform = [[f32; 4]; 4];
