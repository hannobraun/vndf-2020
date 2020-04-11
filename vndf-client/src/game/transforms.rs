use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Rect,
    },
};

use crate::{
    camera::Camera,
    draw::Transform,
    game::coords::{
        Screen,
        World,
    },
    shared::world::math::{
        Pnt2,
        Vec2,
    },
};


pub struct ScreenTransform;

impl Transform for ScreenTransform {
    type Point = Screen<Pnt2>;

    fn enable(&self, context: &mut Context) -> GameResult {
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
}


pub struct WorldTransform<'r>(pub &'r Camera);

impl Transform for WorldTransform<'_> {
    type Point = World<Pnt2>;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) = graphics::drawable_size(context);
        let screen_size = Screen(Vec2::new(screen_width, screen_height));

        let camera = self.0;

        let size       = camera.world_size_on_screen(screen_size);
        let upper_left = camera.center - size / 2.0;

        graphics::set_screen_coordinates(
            context,
            Rect {
                x: upper_left.0.x,
                y: upper_left.0.y,
                w: size.0.x,
                h: size.0.y,
            },
        )?;

        Ok(())
    }
}
