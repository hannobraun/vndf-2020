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
    State,
    components::{
        Body,
        Missile,
        Ship,
    },
};


pub struct Graphics {
    boundary: Mesh,
    missile:  Mesh,
    ship:     Mesh,
}

impl Graphics {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        activate_world_coordinate_system(context)?;

        let boundary = Mesh::new_polygon(
            context,
            DrawMode::stroke(3.0 / WORLD_SIZE),
            &[
                [ 0.5,  0.5],
                [ 0.5, -0.5],
                [-0.5, -0.5],
                [-0.5,  0.5],
            ],
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
        let missile = Mesh::new_polygon(
            context,
            DrawMode::fill(),
            &[
                [ 0.5,  0.5],
                [ 0.5, -0.5],
                [-0.5, -0.5],
                [-0.5,  0.5],
            ],
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;
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
                boundary,
                missile,
                ship,
            }
        )
    }

    pub fn draw(&self, context: &mut Context, state: &State) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        self.draw_world(context, state)?;

        graphics::present(context)?;
        Ok(())
    }

    fn draw_world(&self, context: &mut Context, state: &State) -> GameResult {
        self.draw_boundary(context)?;

        for (_, (body, _)) in &mut state.world.query::<(&Body, &Ship)>() {
            self.draw_ship(context, body)?;
        }
        for (_, (body, _)) in &mut state.world.query::<(&Body, &Missile)>() {
            self.draw_missile(context, body)?;
        }

        Ok(())
    }

    fn draw_boundary(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.boundary,
            DrawParam::new()
                .scale([WORLD_SIZE, WORLD_SIZE])
        )
    }

    fn draw_ship(&self, context: &mut Context, body: &Body) -> GameResult {
        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest(body.pos)
                .rotation(body.dir.0)
                .scale([30.0, 30.0]),
        )
    }

    fn draw_missile(&self, context: &mut Context, body: &Body) -> GameResult {
        graphics::draw(
            context,
            &self.missile,
            DrawParam::new()
                .dest(body.pos)
                .scale([4.0, 4.0])
        )
    }
}


fn activate_world_coordinate_system(context: &mut Context) -> GameResult {
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
