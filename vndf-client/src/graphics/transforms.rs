use crate::{
    camera::Camera,
    graphics::{
        self,
        math::Pixel,
    },
    shared::world::math::Meter,
};


pub type Transform<Src, Dest> = euclid::Transform2D<f32, Src, Dest>;


pub fn world_to_screen(
    camera:           &Camera,
    screen_size:      graphics::Size,
    pixels_per_meter: f32,
)
    -> Transform<Meter, Pixel>
{
    Transform::identity()
        .pre_translate(-camera.center.to_vector())
        .post_scale(pixels_per_meter, pixels_per_meter)
        .post_translate(screen_size.to_vector() / 2.0)
}
