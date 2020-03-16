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
}


pub fn screen_to_world(context: &mut Context, point: Pnt2) -> Pnt2 {
    let (width, height) = graphics::drawable_size(context);
    let middle_centered = point - Vec2::new(width / 2.0, height / 2.0);

    let world_rect = default_world_size_on_screen(context);
    let point_world = Pnt2::new(
        middle_centered.x * world_rect[0] / width,
        middle_centered.y * world_rect[1] / height,
    );

    point_world
}


pub fn activate_world_coordinate_system(
    context: &mut Context,
    camera:  &Camera,
)
    -> GameResult
{
    let size = default_world_size_on_screen(context)
        / camera.zoom;

    let upper_left = camera.center - size / 2.0;

    graphics::set_screen_coordinates(
        context,
        Rect {
            x: upper_left.x,
            y: upper_left.y,
            w: size[0],
            h: size[1],
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


fn default_world_size_on_screen(context: &mut Context) -> Vec2 {
    let (width, height) = graphics::drawable_size(context);
    let aspect_ratio = width / height;

    let default_size = 1000.0;

    if aspect_ratio >= 1.0 {
        Vec2::new(default_size * aspect_ratio, default_size)
    }
    else {
        Vec2::new(default_size, default_size / aspect_ratio)
    }
}
