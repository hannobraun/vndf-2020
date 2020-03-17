use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
};

use crate::shared::math::{
    prelude::*,
    Pnt2,
    Vec2,
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
        point_screen: Pnt2,
    )
        -> Pnt2
    {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let point_screen_origin_centered =
            point_screen - Vec2::new(screen_width / 2.0, screen_height / 2.0);

        let world_rect = self.world_size_on_screen(context);
        let point_world = Pnt2::new(
            point_screen_origin_centered.x * world_rect.x / screen_width,
            point_screen_origin_centered.y * world_rect.y / screen_height,
        );

        point_world + self.center.to_vec()
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


pub fn activate_world_coordinate_system(
    context: &mut Context,
    camera:  &Camera,
)
    -> GameResult
{
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

pub fn activate_screen_coordinate_system(context: &mut Context) -> GameResult {
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
