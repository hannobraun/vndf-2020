use ggez::{
    self,
    Context,
    GameResult,
};

use crate::{
    camera::Camera,
    draw::Transform,
    graphics::{
        self,
        WorldElement,
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
    pub element: WorldElement,
    pub camera:  &'r Camera,
}

impl Transform for WorldTransform<'_> {
    type Point  = world::Pnt2;
    type Vector = world::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) =
            ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(screen_width, screen_height);

        let transform = transforms::local_to_world(&self.element)
            .post_transform(
                &transforms::world_to_screen(self.camera, screen_size)
            )
            .post_transform(
                &transforms::screen_to_homogeneous(screen_size)
            )
            .to_3d()
            .to_row_arrays();

        ggez::graphics::set_projection(context, transform);
        ggez::graphics::apply_transformations(context)?;

        Ok(())
    }
}
