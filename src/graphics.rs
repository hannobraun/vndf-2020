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

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest([0.0, 0.0])
                .scale([50.0, 50.0]),
        )?;

        graphics::present(context)?;
        Ok(())
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
        x: -size[0] / 2.0,
        y: -size[1] / 2.0,
        w: size[0],
        h: size[1],
    };

    graphics::set_screen_coordinates(context, screen_coordinates)?;

    Ok(())
}
