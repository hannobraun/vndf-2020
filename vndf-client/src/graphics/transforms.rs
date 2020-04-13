use crate::{
    camera::Camera,
    graphics::{
        self,
        WorldElement,
        math::{
            ClipUnit,
            ModelUnit,
            Pixel,
        },
    },
    shared::world::math::Meter,
};


pub type Transform<Src, Dest> = euclid::Transform2D<f32, Src, Dest>;


/// Returns what is commonly known as the model matrix
pub fn local_to_world(element: &WorldElement) -> Transform<ModelUnit, Meter> {
    Transform::identity()
        .post_scale(element.size.width, element.size.height)
        .post_rotate(element.angle)
        .post_translate(element.pos.to_vector())
}

/// Returns what is commonly known as the view matrix
pub fn world_to_screen(camera: &Camera, screen_size: graphics::Size)
    -> Transform<Meter, Pixel>
{
    let pixels_per_meter = camera.pixels_per_meter(screen_size);

    Transform::identity()
        .pre_translate(-camera.center.to_vector())
        .post_scale(pixels_per_meter, pixels_per_meter)
        .post_translate(screen_size.to_vector() / 2.0)
}

/// Returns what is commonly known as the projection matrix
pub fn screen_to_homogeneous(screen_size: graphics::Size)
    -> Transform<Pixel, ClipUnit>
{
    let clip_units_per_pixel = graphics::Vec2::new(
        2.0 / screen_size.width,
        2.0 / screen_size.height,
    );

    Transform::identity()
        .pre_scale(1.0, -1.0)
        .pre_scale(clip_units_per_pixel.x, clip_units_per_pixel.y)
        .pre_translate(-screen_size.to_vector() / 2.0)
}


#[cfg(test)]
mod tests {
    use crate::graphics;

    use super::screen_to_homogeneous;


    #[test]
    fn test_screen_to_homogeneous() {
        let screen_size = graphics::Size::new(100.0, 200.0);
        let transform   = screen_to_homogeneous(screen_size);

        assert_eq!(
            transform.transform_point(graphics::Pnt2::new(0.0, 0.0)),
            euclid::Point2D::new(-1.0, 1.0),
        );
        assert_eq!(
            transform.transform_point(graphics::Pnt2::new(100.0, 200.0)),
            euclid::Point2D::new(1.0, -1.0),
        );
        assert_eq!(
            transform.transform_point(graphics::Pnt2::new(50.0, 100.0)),
            euclid::Point2D::new(0.0, 0.0),
        );
    }
}
