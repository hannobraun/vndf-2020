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
    pub fn color(self, color: impl Into<Color>) -> Self {
        Self(self.0.color(color.into()), PhantomData, PhantomData)
    }
}

impl<P, V> From<DrawParam<P, V>> for ggez::graphics::DrawParam {
    fn from(from: DrawParam<P, V>) -> Self {
        from.0
    }
}
