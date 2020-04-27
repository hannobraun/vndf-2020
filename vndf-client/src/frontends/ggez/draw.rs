use ggez::{
    Context,
    GameResult,
    graphics::Drawable,
};


pub fn draw<D>(
    context:   &mut Context,
    transform: [[f32; 4]; 4],
    drawable:  &D,
    color:     Option<[f32; 4]>,
)
    -> GameResult
    where
        D: Drawable,
{
    ggez::graphics::set_projection(context, transform);
    ggez::graphics::apply_transformations(context)?;

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
