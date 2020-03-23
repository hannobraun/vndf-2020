use core::ops::{
    Add,
    Div,
    Sub,
};

use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
    mint,
};

use crate::{
    draw::Transform,
    shared::{
        game::physics::Position,
        math::{
            prelude::*,
            Pnt2,
            Vec2,
        },
    },
};


pub struct Camera {
    pub center: World<Pnt2>,
    pub zoom:   f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: World(Pnt2::new(0.0, 0.0)),
            zoom:   1.0,
        }
    }

    pub fn screen_to_world(&self,
        context:      &mut Context,
        point_screen: Screen<Pnt2>,
    )
        -> World<Pnt2>
    {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        let point_screen_origin_centered = point_screen - screen_size / 2.0;

        let world_rect = self.world_size_on_screen(context);
        let point_world = World(
            Pnt2::new(
                point_screen_origin_centered.0.x
                    * world_rect.0.x
                    / screen_width,
                point_screen_origin_centered.0.y
                    * world_rect.0.y
                    / screen_height,
            )
        );

        point_world + self.center.to_vec()
    }

    pub fn world_to_screen(&self,
        context:     &mut Context,
        point_world: impl Into<World<Pnt2>>,
    )
        -> Screen<Pnt2>
    {
        let point_world = point_world.into();

        let (screen_width, screen_height) = graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        let point_camera = point_world - self.center.to_vec();

        let world_rect = self.world_size_on_screen(context);
        let point_screen_origin_centered = Screen(
            Pnt2::new(
                point_camera.0.x * screen_width  / world_rect.0.x,
                point_camera.0.y * screen_height / world_rect.0.y,
            )
        );

        point_screen_origin_centered + screen_size / 2.0
    }

    pub fn world_size_on_screen(&self, context: &Context) -> World<Vec2> {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let aspect_ratio = screen_width / screen_height;

        let min_world_size_on_screen = 1000.0;

        let default_world_size_on_screen = World(
            if aspect_ratio >= 1.0 {
                Vec2::new(
                    min_world_size_on_screen * aspect_ratio,
                    min_world_size_on_screen,
                )
            }
            else {
                Vec2::new(
                    min_world_size_on_screen,
                    min_world_size_on_screen / aspect_ratio,
                )
            }
        );

        default_world_size_on_screen / self.zoom
    }
}


pub struct ScreenTransform;

impl Transform for ScreenTransform {
    type Point = Screen<Pnt2>;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (width, height) = graphics::drawable_size(context);

        graphics::set_screen_coordinates(
            context,
            Rect {
                x: 0.0,
                y: 0.0,
                w: width,
                h: height,
            },
        )?;

        Ok(())
    }
}


pub struct WorldTransform<'r>(pub &'r Camera);

impl Transform for WorldTransform<'_> {
    type Point = World<Pnt2>;

    fn enable(&self, context: &mut Context) -> GameResult {
        let camera = self.0;

        let size       = camera.world_size_on_screen(context);
        let upper_left = camera.center - size / 2.0;

        graphics::set_screen_coordinates(
            context,
            Rect {
                x: upper_left.0.x,
                y: upper_left.0.y,
                w: size.0.x,
                h: size.0.y,
            },
        )?;

        Ok(())
    }
}


macro_rules! coord_wrappers {
    ($($name:ident,)*) => {
        $(
            #[derive(Clone, Copy)]
            pub struct $name<T>(pub T);

            impl $name<Pnt2> {
                pub fn to_vec(self) -> $name<Vec2> {
                    $name(self.0.to_vec())
                }
            }

            impl From<Pnt2> for $name<Pnt2> {
                fn from(from: Pnt2) -> Self {
                    Self(from)
                }
            }

            impl From<$name<Pnt2>> for mint::Point2<f32> {
                fn from(from: $name<Pnt2>) -> Self {
                    from.0.into()
                }
            }

            impl Add<$name<Vec2>> for $name<Pnt2> {
                type Output = Self;

                fn add(self, rhs: $name<Vec2>) -> Self::Output {
                    $name(self.0 + rhs.0)
                }
            }

            impl Sub<$name<Vec2>> for $name<Pnt2> {
                type Output = Self;

                fn sub(self, rhs: $name<Vec2>) -> Self::Output {
                    $name(self.0 - rhs.0)
                }
            }

            impl Div<f32> for $name<Vec2> {
                type Output = Self;

                fn div(self, rhs: f32) -> Self::Output {
                    $name(self.0 / rhs)
                }
            }
        )*
    };
}

coord_wrappers!(
    Screen,
    World,
);

impl From<Position> for World<Pnt2> {
    fn from(from: Position) -> Self {
        Self(from.0)
    }
}

impl From<&'_ Position> for World<Pnt2> {
    fn from(from: &Position) -> Self {
        Self(from.0)
    }
}
