use ggez::{
    self,
    Context,
    GameResult,
};

use crate::{
    draw::Transform,
    game::camera::Camera,
    graphics::{
        self,
        elements::{
            UiElement,
            WorldElement,
        },
    },
    shared::world,
};


pub struct ScreenTransform<'r> {
    pub element: &'r UiElement,
}

impl Transform for ScreenTransform<'_> {
    type Point  = graphics::Pnt2;
    type Vector = graphics::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(width, height);

        let transform = self.element.transform(screen_size);

        ggez::graphics::set_projection(context, transform);
        ggez::graphics::apply_transformations(context)?;

        Ok(())
    }
}


pub struct WorldTransform<'r> {
    pub element: &'r WorldElement,
    pub camera:  &'r Camera,
}

impl Transform for WorldTransform<'_> {
    type Point  = world::Pnt2;
    type Vector = world::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (screen_width, screen_height) =
            ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(screen_width, screen_height);

        let transform = self.element.transform(self.camera, screen_size);

        ggez::graphics::set_projection(context, transform);
        ggez::graphics::apply_transformations(context)?;

        Ok(())
    }
}
