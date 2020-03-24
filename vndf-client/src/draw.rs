use core::marker::PhantomData;

use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Color,
        Drawable,
    },
    mint,
};

use crate::{
    game::transforms::{
        Screen,
        World,
    },
    shared::math::{
        Pnt2,
        Vec2,
    },
};


pub fn draw<T, D>(
    context:   &mut Context,
    transform: &T,
    drawable:  &D,
    params:    DrawParam<T::Point>,
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
    type Point;

    fn enable(&self, _: &mut Context) -> GameResult;
}


pub struct DrawParam<P>(graphics::DrawParam, PhantomData<P>);

impl DrawParam<Screen<Pnt2>> {
    pub fn screen() -> Self {
        Self(graphics::DrawParam::new(), PhantomData)
    }
}

impl DrawParam<World<Pnt2>> {
    pub fn world() -> Self {
        Self(graphics::DrawParam::new(), PhantomData)
    }
}

impl<P> DrawParam<P>
    where
        P: Into<mint::Point2<f32>>
{
    pub fn dest(self, dest: impl Into<P>) -> Self {
        Self(self.0.dest(dest.into()), PhantomData)
    }

    pub fn scale(self, scale: Vec2) -> Self {
        Self(self.0.scale(scale), PhantomData)
    }

    pub fn rotation(self, rotation: f32) -> Self {
        Self(self.0.rotation(rotation), PhantomData)
    }

    pub fn color(self, color: impl Into<Color>) -> Self {
        Self(self.0.color(color.into()), PhantomData)
    }
}

impl<P> From<DrawParam<P>> for graphics::DrawParam {
    fn from(from: DrawParam<P>) -> Self {
        from.0
    }
}
