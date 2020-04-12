use ggez::{
    self,
    Context,
    GameResult,
    graphics::Rect,
};

use crate::{
    camera::Camera,
    draw::Transform,
    graphics::{
        self,
        transforms,
    },
    shared::world,
};


pub struct ScreenTransform;

impl Transform for ScreenTransform {
    type Point  = graphics::Pnt2;
    type Vector = graphics::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(width, height);

        let transform = transforms::screen_to_homogeneous(screen_size)
            .to_3d()
            .to_row_arrays();

        ggez::graphics::set_projection(context, transform);
        ggez::graphics::apply_transformations(context)?;

        Ok(())
    }
}


pub struct WorldTransform<'r> {
    pub camera: &'r Camera,
}

impl Transform for WorldTransform<'_> {
    type Point  = world::Pnt2;
    type Vector = world::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) =
            ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(screen_width, screen_height);

        let size       = self.camera.world_size_on_screen(screen_size);
        let upper_left = self.camera.center - size / 2.0;

        ggez::graphics::set_screen_coordinates(
            context,
            Rect {
                x: upper_left.x,
                y: upper_left.y,
                w: size.width,
                h: size.height,
            },
        )?;

        Ok(())
    }
}
