use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Color,
        Drawable,
    }
};

use crate::shared::math::{
    Pnt2,
    Vec2,
};


pub fn draw<T, D>(
    context:   &mut Context,
    transform: &T,
    drawable:  &D,
    params:    DrawParam,
)
    -> GameResult
    where
        T: Transform,
        D: Drawable,
{
    transform.enable(context)?;

    graphics::draw(
        context,
        drawable,
        params,
    )
}


pub trait Transform {
    fn enable(&self, _: &mut Context) -> GameResult;
}


pub struct DrawParam(graphics::DrawParam);

impl DrawParam {
    pub fn new() -> Self {
        Self(graphics::DrawParam::new())
    }

    pub fn dest(self, dest: Pnt2) -> Self {
        Self(self.0.dest(dest))
    }

    pub fn scale(self, scale: Vec2) -> Self {
        Self(self.0.scale(scale))
    }

    pub fn rotation(self, rotation: f32) -> Self {
        Self(self.0.rotation(rotation))
    }

    pub fn color(self, color: impl Into<Color>) -> Self {
        Self(self.0.color(color.into()))
    }
}

impl From<DrawParam> for graphics::DrawParam {
    fn from(from: DrawParam) -> Self {
        from.0
    }
}
