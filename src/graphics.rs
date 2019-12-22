use cgmath::{
    Matrix4,
    Ortho,
};
use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        DrawMode,
        DrawParam,
        Mesh,
    },
};

use crate::{
    math::Pnt2,
    state::{
        Body,
        State,
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

    pub fn draw(&self, context: &mut Context, state: &State) -> GameResult {
        graphics::clear(context, [0.0, 0.0, 0.1, 1.0].into());

        for (_, (body,)) in &mut state.world.query::<(&Body,)>() {
            self.draw_ship(context, body.pos)?;
        }

        graphics::present(context)?;
        Ok(())
    }

    fn draw_ship(&self, context: &mut Context, pos: Pnt2) -> GameResult {
        graphics::draw(
            context,
            &self.ship,
            DrawParam::new()
                .dest(pos)
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

    let transform = Ortho {
        left:   -min_size / 2.0,
        right:  -min_size / 2.0 + size[0],
        bottom: -min_size / 2.0 + size[1],
        top:    -min_size / 2.0,
        near:   -1.0,
        far:    1.0,
    };
    let transform: Matrix4<_> = transform.into();

    graphics::set_projection(context, transform);
    graphics::apply_transformations(context)?;

    Ok(())
}
