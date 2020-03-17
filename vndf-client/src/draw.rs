use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Drawable,
        DrawParam,
    }
};


pub fn draw<T, D, P>(
    context:   &mut Context,
    transform: &T,
    drawable:  &D,
    params:    P
)
    -> GameResult
    where
        T: Transform,
        D: Drawable,
        P: Into<DrawParam>,
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
