use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Drawable,
        DrawParam,
    }
};


pub fn draw<D, T>(
    context: &mut Context,
    drawable: &D,
    params:   T
)
    -> GameResult
    where
        D: Drawable,
        T: Into<DrawParam>,
{
    graphics::draw(
        context,
        drawable,
        params,
    )
}
