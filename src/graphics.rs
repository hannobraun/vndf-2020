use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
        Rect,
    },
};

use crate::world::World;


pub struct Graphics {
    ship: Mesh,
}

impl Graphics {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        set_coordinate_system(context)?;

        let ship = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            &[
                [ 0.5,  0.0],
                [-0.5,  0.4],
                [-0.2,  0.0],
                [-0.5, -0.4],
            ],
            [1.0, 1.0, 0.0, 1.0].into(),
        )?;

        Ok(
            Graphics {
                ship,
            }
        )
    }

    pub fn draw(&self, context: &mut Context, world: &World) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        self.draw_ship(context, world)?;

        graphics::present(context)?;
        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, world: &World) -> GameResult {
        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest(world.position)
                .scale([50.0, 50.0]),
        )
    }
}


fn set_coordinate_system(context: &mut Context) -> GameResult {
    let (width, height) = graphics::drawable_size(context);
    let aspect_ratio = width / height;

    let min_size = 1000.0;

    let size = if aspect_ratio >= 1.0 {
        [min_size * aspect_ratio, min_size]
    }
    else {
        [min_size, min_size / aspect_ratio]
    };

    let screen_coordinates = Rect {
        x: -min_size / 2.0,
        y: -min_size / 2.0,
        w: size[0],
        h: size[1],
    };

    graphics::set_screen_coordinates(context, screen_coordinates)?;

    Ok(())
}
