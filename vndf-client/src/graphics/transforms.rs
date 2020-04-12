use crate::{
    camera::Camera,
    graphics::{
        self,
        math::Pixel,
    },
    shared::world::math::Meter,
};


pub type Transform<Src, Dest> = euclid::Transform2D<f32, Src, Dest>;


/// Returns what is commonly known as the view matrix
pub fn world_to_screen(camera: &Camera, screen_size: graphics::Size,
)
    -> Transform<Meter, Pixel>
{
    let pixels_per_meter = camera.pixels_per_meter(screen_size);

    Transform::identity()
        .pre_translate(-camera.center.to_vector())
        .post_scale(pixels_per_meter, pixels_per_meter)
        .post_translate(screen_size.to_vector() / 2.0)
}
