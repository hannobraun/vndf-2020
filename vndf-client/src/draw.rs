use ggez::{
    Context,
    GameResult,
    graphics::{
        self,
        Drawable,
        DrawParam,
    }
};


pub fn draw<D, P>(
    context: &mut Context,
    drawable: &D,
    params:   P
)
    -> GameResult
    where
        D: Drawable,
        P: Into<DrawParam>,
{
    graphics::draw(
        context,
        drawable,
        params,
    )
}
