use ggez::{
    self,
    Context,
    GameResult,
    graphics::Rect,
};

use crate::{
    camera::Camera,
    draw::Transform,
    graphics,
    shared::world,
};


pub struct ScreenTransform;

impl Transform for ScreenTransform {
    type Point  = graphics::Pnt2;
    type Vector = graphics::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(context);

        ggez::graphics::set_screen_coordinates(
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
    type Point  = world::Pnt2;
    type Vector = world::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) =
            ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(screen_width, screen_height);

        let camera = self.0;

        let size       = camera.world_size_on_screen(screen_size);
        let upper_left = camera.center - size / 2.0;

        ggez::graphics::set_screen_coordinates(
            context,
            Rect {
                x: upper_left.x,
                y: upper_left.y,
                w: size.x,
                h: size.y,
            },
        )?;

        Ok(())
    }
}
