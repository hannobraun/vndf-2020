use crate::{
    game::{
        Game,
        camera::Camera,
    },
    graphics::{
        self,
        math::{
            ClipUnit,
            LocalUnit,
        },
        screen::Screen,
        transforms::{
            self,
            Transform,
        },
    },
    shared::world::{
        self,
        behavior::{
            crafts::Craft,
            explosions::Explosion,
            orbits::Orbit,
            physics::Position,
            planets::Planet,
            ships::Ship,
        },
    },
};


pub struct ScreenElement {
    pub size:  graphics::Size,
    pub pos:   graphics::Pnt2,
    pub angle: graphics::Angle,
}

impl ScreenElement {
    pub fn from_ship(
        ship:   &Ship,
        game:   &Game,
        screen: &Screen,
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
        screen: &Screen,
    )
        -> Option<Self>
    {
        let body = game.state.data.bodies.get(&craft.body)?;
        let pos  = game.state.data.positions.get(&body.pos)?;

        Some(
            Self::from_pos(
                pos,
                body.dir,
                size,
                game,
                screen,
            )
        )
    }

    pub fn from_explosion(
        explosion: &Explosion,
        game:      &Game,
        screen:    &Screen,
    )
        -> Option<Self>
    {
        let pos = game.state.data.positions.get(&explosion.pos)?;

        let size = explosion.strength_total * 2.0;
        let size = size as graphics::Scalar;

        Some(
            Self::from_pos(
                pos,
                world::Vec2::new(1.0, 0.0),
                graphics::Size::new(size, size),
                game,
                screen,
            )
        )
    }

    pub fn from_pos(
        pos:    &Position,
        dir:    world::Vec2,
        size:   graphics::Size,
        game:   &Game,
        screen: &Screen,
    )
        -> Self
    {
        let size = size * screen.scale_factor;
        let pos = transforms::world_to_screen(&game.state.camera, screen.size).0
            .transform_point(pos.0.cast());
        let angle = dir.angle_from_x_axis();

        // Can be replaced with `.cast()`, once this PR lands:
        // https://github.com/servo/euclid/pull/440
        let angle = graphics::Angle {
            radians: angle.radians as graphics::Scalar,
        };

        Self {
            size,
            pos,
            angle,
        }
    }

    pub fn transform(&self, screen_size: graphics::Size)
        -> Transform<LocalUnit, ClipUnit>
    {
        transforms::local_to_screen(self)
            .post_transform(
                &transforms::screen_to_homogeneous(screen_size)
            )
    }
}

impl Default for ScreenElement {
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
        -> Transform<LocalUnit, ClipUnit>
    {
        transforms::local_to_world(self)
            .post_transform(
                &transforms::world_to_screen(camera, screen_size)
            )
            .post_transform(
                &transforms::screen_to_homogeneous(screen_size)
            )
    }
}

impl From<&Orbit> for WorldElement {
    fn from(orbit: &Orbit) -> Self {
        let size = world::Size::from_lengths(
            orbit.semi_major_axis,
            orbit.semi_minor_axis,
        );
        let pos = orbit.ellipse_pos;
        let angle = orbit.arg_of_periapsis;

        Self {
            size,
            pos,
            angle,
        }
    }
}

impl From<&Planet> for WorldElement {
    fn from(planet: &Planet) -> Self {
        Self {
            size:  world::Size::from_lengths(planet.radius, planet.radius),
            pos:   planet.pos,
            angle: world::Angle::zero(),
        }
    }
}
