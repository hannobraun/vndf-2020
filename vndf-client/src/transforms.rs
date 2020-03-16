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


pub fn screen_to_world(context: &mut Context, point: Pnt2) -> Pnt2 {
    let (width, height) = graphics::drawable_size(context);
    let middle_centered = point - Vec2::new(width / 2.0, height / 2.0);

    let world_rect = world_rect(context);
    let point_world = Pnt2::new(
        middle_centered.x * world_rect[0] / width,
        middle_centered.y * world_rect[1] / height,
    );

    point_world
}


pub fn activate_world_coordinate_system(
    context: &mut Context,
    center:  Pnt2,
    zoom:    f32,
)
    -> GameResult
{
    let size = world_rect(context);

    let size = [
        size[0] / zoom,
        size[1] / zoom,
    ];

    graphics::set_screen_coordinates(
        context,
        Rect {
            x: center.x - size[0] / 2.0,
            y: center.y - size[1] / 2.0,
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


fn world_rect(context: &mut Context) -> [f32; 2] {
    let (width, height) = graphics::drawable_size(context);
    let aspect_ratio = width / height;

    let default_size = 1000.0;

    if aspect_ratio >= 1.0 {
        [default_size * aspect_ratio, default_size]
    }
    else {
        [default_size, default_size / aspect_ratio]
    }
}
