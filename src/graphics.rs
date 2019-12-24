use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
        Rect,
        Text,
    },
};

use crate::state::{
    WORLD_SIZE,
    State,
    components::{
        Body,
        Engine,
        Explosion,
        Missile,
        Ship,
    },
};


pub struct Graphics {
    boundary:  Mesh,
    explosion: Mesh,
    missile:   Mesh,
    ship:      Mesh,
}

impl Graphics {
    pub fn new(context: &mut Context) -> GameResult<Self> {
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
        let explosion = Mesh::new_circle(
            context,
            DrawMode::fill(),
            [0.0, 0.0],
            1.0,
            0.01,
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
                explosion,
                missile,
                ship,
            }
        )
    }

    pub fn draw(&self, context: &mut Context, state: &State) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        self.draw_world(context, state)?;
        self.draw_ui(context, state)?;

        graphics::present(context)?;
        Ok(())
    }

    fn draw_world(&self, context: &mut Context, state: &State) -> GameResult {
        activate_world_coordinate_system(context)?;

        self.draw_boundary(context)?;

        for (_, (_, body)) in &mut state.world.query::<(&Ship, &Body)>() {
            self.draw_ship(context, body)?;
        }

        for (_, (_, body)) in &mut state.world.query::<(&Missile, &Body)>() {
            self.draw_missile(context, body)?;
        }

        let query = &mut state.world.query::<(&Explosion, &Body)>();
        for (_, (explosion, body)) in query {
            self.draw_explosion(context, explosion, body)?;
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

    fn draw_explosion(&self,
        context:   &mut Context,
        explosion: &Explosion,
        body:      &Body,
    )
        -> GameResult
    {
        let alpha = explosion.time_left / explosion.time_total;

        graphics::draw(
            context,
            &self.explosion,
            DrawParam::new()
                .dest(body.pos)
                .scale([6.0, 6.0])
                .color([1.0, 1.0, 1.0, alpha].into())
        )
    }

    fn draw_ui(&self, context: &mut Context, state: &State) -> GameResult {
        activate_ui_coordinate_system(context)?;

        for (_, (_, engine)) in &mut state.world.query::<(&Ship, &Engine)>() {
            let (width, _) = graphics::drawable_size(context);

            graphics::draw(
                context,
                &Text::new(format!("Fuel: {:.2}", engine.fuel)),
                DrawParam::new()
                    .dest([width - 200.0, 20.0])
            )?;

            // There should only be one ship, so let's quit.
            return Ok(());
        }

        Ok(())
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

fn activate_ui_coordinate_system(context: &mut Context) -> GameResult {
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
