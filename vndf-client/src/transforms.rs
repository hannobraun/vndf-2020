use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
};

use crate::shared::math::{
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

    pub fn world_size_on_screen(&self, context: &Context) -> Vec2 {
        default_world_size_on_screen(context) / self.zoom
    }
}


pub fn screen_to_world(context: &mut Context, screen_point: Pnt2) -> Pnt2 {
    let (screen_width, screen_height) = graphics::drawable_size(context);
    let middle_centered_screen_point =
        screen_point - Vec2::new(screen_width / 2.0, screen_height / 2.0);

    let world_rect = default_world_size_on_screen(context);
    let point_world = Pnt2::new(
        middle_centered_screen_point.x * world_rect[0] / screen_width,
        middle_centered_screen_point.y * world_rect[1] / screen_height,
    );

    point_world
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

pub fn activate_ui_coordinate_system(context: &mut Context) -> GameResult {
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


fn default_world_size_on_screen(context: &Context) -> Vec2 {
    let (screen_width, screen_height) = graphics::drawable_size(context);
    let aspect_ratio = screen_width / screen_height;

    let min_world_size_on_screen = 1000.0;

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
}
