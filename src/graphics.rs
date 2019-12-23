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

use crate::state::{
    WORLD_SIZE,
    Body,
    State,
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
                [ 0.6,  0.0],
                [-0.4,  0.4],
                [-0.1,  0.0],
                [-0.4, -0.4],
            ],
            [1.0, 1.0, 0.0, 1.0].into(),
        )?;

        Ok(
            Graphics {
                ship,
            }
        )
    }

    pub fn draw(&self, context: &mut Context, state: &State) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        for (_, (body,)) in &mut state.world.query::<(&Body,)>() {
            self.draw_ship(context, body)?;
        }

        graphics::present(context)?;
        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, body: &Body) -> GameResult {
        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest(body.pos)
                .rotation(body.dir.0)
                .scale([50.0, 50.0]),
        )
    }
}


fn set_coordinate_system(context: &mut Context) -> GameResult {
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
