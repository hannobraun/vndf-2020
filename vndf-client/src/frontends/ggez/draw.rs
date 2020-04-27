use core::marker::PhantomData;

use ggez::{
    Context,
    GameResult,
    graphics::{
        Color,
        Drawable,
    },
    mint,
};

use crate::{
    graphics,
    shared::world,
};


pub fn draw<T, D>(
    context:   &mut Context,
    transform: &T,
    drawable:  &D,
    params:    DrawParam<T::Point, T::Vector>,
)
    -> GameResult
    where
        T: Transform,
        D: Drawable,
{
    transform.enable(context)?;

    ggez::graphics::draw(
        context,
        drawable,
        params,
    )
}


pub trait Transform {
    type Point;
    type Vector;

    fn enable(&self, _: &mut Context) -> GameResult;
}


pub struct DrawParam<P, V>(
    ggez::graphics::DrawParam,
    PhantomData<P>,
    PhantomData<V>,
);

impl DrawParam<graphics::Pnt2, graphics::Vec2> {
    pub fn screen() -> Self {
        Self(ggez::graphics::DrawParam::new(), PhantomData, PhantomData)
    }
}

impl DrawParam<world::Pnt2, world::Vec2> {
    pub fn world() -> Self {
        Self(ggez::graphics::DrawParam::new(), PhantomData, PhantomData)
    }
}

impl<P, V> DrawParam<P, V>
    where
        P: Into<mint::Point2<f32>>,
        V: Into<mint::Vector2<f32>>,
{
    pub fn dest(self, dest: impl Into<P>) -> Self {
        Self(self.0.dest(dest.into()), PhantomData, PhantomData)
    }

    pub fn scale(self, scale: impl Into<V>) -> Self {
        Self(self.0.scale(scale.into()), PhantomData, PhantomData)
    }

    pub fn color(self, color: impl Into<Color>) -> Self {
        Self(self.0.color(color.into()), PhantomData, PhantomData)
    }
}

impl<P, V> From<DrawParam<P, V>> for ggez::graphics::DrawParam {
    fn from(from: DrawParam<P, V>) -> Self {
        from.0
    }
}
