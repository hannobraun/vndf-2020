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
};

use crate::{
    draw::Transform,
    shared::math::{
        prelude::*,
        Pnt2,
        Vec2,
    },
};


pub struct Camera {
    pub center: Pnt2,
    pub zoom:   f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: Pnt2::new(0.0, 0.0),
            zoom:   1.0,
        }
    }

    pub fn screen_to_world(&self,
        context:      &mut Context,
        point_screen: Screen<Pnt2>,
    )
        -> Pnt2
    {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        let point_screen_origin_centered = point_screen - screen_size / 2.0;

        let world_rect = self.world_size_on_screen(context);
        let point_world = Pnt2::new(
            point_screen_origin_centered.0.x * world_rect.x / screen_width,
            point_screen_origin_centered.0.y * world_rect.y / screen_height,
        );

        point_world + self.center.to_vec()
    }

    pub fn world_to_screen(&self,
        context:     &mut Context,
        point_world: Pnt2,
    )
        -> Screen<Pnt2>
    {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        let point_camera = point_world - self.center.to_vec();

        let world_rect = self.world_size_on_screen(context);
        let point_screen_origin_centered = Screen(
            Pnt2::new(
                point_camera.x * screen_width  / world_rect.x,
                point_camera.y * screen_height / world_rect.y,
            )
        );

        point_screen_origin_centered + screen_size / 2.0
    }

    pub fn world_size_on_screen(&self, context: &Context) -> Vec2 {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let aspect_ratio = screen_width / screen_height;

        let min_world_size_on_screen = 1000.0;

        let default_world_size_on_screen = if aspect_ratio >= 1.0 {
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
        };

        default_world_size_on_screen / self.zoom
    }
}


pub struct ScreenTransform;

impl Transform for ScreenTransform {
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
    fn enable(&self, context: &mut Context) -> GameResult {
        let camera = self.0;

        let size       = camera.world_size_on_screen(context);
        let upper_left = camera.center - size / 2.0;

        graphics::set_screen_coordinates(
            context,
            Rect {
                x: upper_left.x,
                y: upper_left.y,
                w: size.x,
                h: size.y,
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
);
