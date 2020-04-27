use ggez::{
    Context,
    GameResult,
    graphics::Drawable,
};

use crate::{
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


pub fn draw<T, D>(
    context:   &mut Context,
    transform: &T,
    drawable:  &D,
    color:     Option<[f32; 4]>,
)
    -> GameResult
    where
        T: Transform,
        D: Drawable,
{
    transform.enable(context)?;

    let mut param = ggez::graphics::DrawParam::new();
    if let Some(color) = color {
        param = param.color(color.into());
    }

    ggez::graphics::draw(
        context,
        drawable,
        param,
    )
}


pub trait Transform {
    type Point;
    type Vector;

    fn enable(&self, _: &mut Context) -> GameResult;
}


pub struct ScreenTransform<'r> {
    pub element: &'r UiElement,
}

impl Transform for ScreenTransform<'_> {
    type Point  = graphics::Pnt2;
    type Vector = graphics::Vec2;

    fn enable(&self, context: &mut Context) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(width, height);

        let transform = self.element
            .transform(screen_size)
            .to_native();

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
        let (width, height) =
            ggez::graphics::drawable_size(context);
        let screen_size = graphics::Size::new(width, height);

        let transform = self.element
            .transform(self.camera, screen_size)
            .to_native();

        ggez::graphics::set_projection(context, transform);
        ggez::graphics::apply_transformations(context)?;

        Ok(())
    }
}
