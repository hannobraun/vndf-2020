use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
};

use crate::shared::game::WORLD_SIZE;


pub fn activate_world_coordinate_system(context: &mut Context) -> GameResult {
    let (width, height) = graphics::drawable_size(context);
    let aspect_ratio = width / height;

    let size = if aspect_ratio >= 1.0 {
        [WORLD_SIZE * aspect_ratio, WORLD_SIZE]
    }
    else {
        [WORLD_SIZE, WORLD_SIZE / aspect_ratio]
    };

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
