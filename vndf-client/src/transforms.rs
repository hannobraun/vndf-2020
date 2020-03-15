use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
};

use crate::{
    input::Input,
    shared::{
        game::WORLD_SIZE,
        math::{
            Pnt2,
            Vec2,
        },
    },
};


pub fn screen_to_world(context: &mut Context, point: Pnt2) -> Option<Pnt2> {
    let (width, height) = graphics::drawable_size(context);
    let middle_centered = point - Vec2::new(width / 2.0, height / 2.0);

    let world_rect = world_rect(context);
    let point_world = Pnt2::new(
        middle_centered.x * world_rect[0] / width,
        middle_centered.y * world_rect[1] / height,
    );

    let in_bounds = point_world.x.abs() < WORLD_SIZE / 2.0
        && point_world.y.abs() < WORLD_SIZE / 2.0;

    if in_bounds {
        Some(point_world)
    }
    else {
        None
    }
}


pub fn activate_world_coordinate_system(context: &mut Context, input: &Input)
    -> GameResult
{
    let size = world_rect(context);

    let size = [
        size[0] / input.zoom,
        size[1] / input.zoom,
    ];

    graphics::set_screen_coordinates(
        context,
        Rect {
            x: -size[0] / 2.0,
            y: -size[1] / 2.0,
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

    if aspect_ratio >= 1.0 {
        [WORLD_SIZE * aspect_ratio, WORLD_SIZE]
    }
    else {
        [WORLD_SIZE, WORLD_SIZE / aspect_ratio]
    }
}
