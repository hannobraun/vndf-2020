use crate::{
    game::camera::Camera,
    graphics::{
        self,
        elements::{
            UiElement,
            WorldElement,
        },
        math::{
            ClipUnit,
            LocalUnit,
            Pixel,
        },
    },
    shared::world::math::Meter,
};


pub struct Transform<Src, Dst>(pub graphics::Transform<Src, Dst>);

impl<Src, Dst> Transform<Src, Dst> {
    pub fn identity() -> Self {
        Self(graphics::Transform::identity())
    }

    pub fn post_transform<NewDst>(&self, transform: &Transform<Dst, NewDst>)
        -> Transform<Src, NewDst>
    {
        Self(self.0.post_transform(&transform.0))
    }

    pub fn to_native(&self) -> [[f32; 4]; 4] {
        self.0
            .to_3d()
            .to_row_arrays()
    }
}

impl<Src, Dst> From<graphics::Transform<Src, Dst>> for Transform<Src, Dst> {
    fn from(transform: graphics::Transform<Src, Dst>) -> Self {
        Self(transform)
    }
}


/// Returns what is commonly known as the model matrix
pub fn local_to_world(element: &WorldElement) -> Transform<LocalUnit, Meter> {
    graphics::Transform::identity()
        .post_scale(element.size.width, element.size.height)
        .post_rotate(element.angle)
        .post_translate(element.pos.to_vector())
        .into()
}

pub fn local_to_screen(element: &UiElement) -> Transform<LocalUnit, Pixel> {
    graphics::Transform::identity()
        .post_scale(element.size.width, element.size.height)
        .post_rotate(element.angle)
        .post_translate(element.pos.to_vector())
        .into()
}

/// Returns what is commonly known as the view matrix
pub fn world_to_screen(camera: &Camera, screen_size: graphics::Size)
    -> Transform<Meter, Pixel>
{
    let pixels_per_meter = camera.pixels_per_meter(screen_size);

    graphics::Transform::identity()
        .pre_translate(-camera.center.to_vector())
        .post_scale(pixels_per_meter, pixels_per_meter)
        .post_translate(screen_size.to_vector() / 2.0)
        .into()
}

/// Returns what is commonly known as the projection matrix
pub fn screen_to_homogeneous(screen_size: graphics::Size)
    -> Transform<Pixel, ClipUnit>
{
    let clip_units_per_pixel = graphics::Vec2::new(
        2.0 / screen_size.width,
        2.0 / screen_size.height,
    );

    graphics::Transform::identity()
        .pre_scale(1.0, -1.0)
        .pre_scale(clip_units_per_pixel.x, clip_units_per_pixel.y)
        .pre_translate(-screen_size.to_vector() / 2.0)
        .into()
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
            transform.0.transform_point(graphics::Pnt2::new(0.0, 0.0)),
            euclid::Point2D::new(-1.0, 1.0),
        );
        assert_eq!(
            transform.0.transform_point(graphics::Pnt2::new(100.0, 200.0)),
            euclid::Point2D::new(1.0, -1.0),
        );
        assert_eq!(
            transform.0.transform_point(graphics::Pnt2::new(50.0, 100.0)),
            euclid::Point2D::new(0.0, 0.0),
        );
    }
}
