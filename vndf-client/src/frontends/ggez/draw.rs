use ggez::{
    Context,
    GameResult,
    graphics::Drawable,
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
