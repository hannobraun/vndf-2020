/// Defines transformations between the various coordinate systems
///
/// The following coordinate systems exist:
/// - **Local space**: The space in which models are defined.
///   - Positive x is right.
///   - Positive y is up.
/// - **World space**:
///   - Positive x is right.
///   - Positive y is up.
///   - Positive angles rotate counter-clockwise.
///   - What the camera points at is drawn at the center of the screen.
/// - **Screen space**:
///   - Positive x is right.
///   - Positive y is down.
///   - The origin is at the upper-left corner.
///   - Positive angles rotate counter-clockwise.
/// - **Homogeneous space**:
///   - Positive x is right.
///   - Positive y is up.
///   - Positive angles rotate counter-clockwise.


use crate::{
    game::camera::Camera,
    graphics::{
        self,
        elements::{
            ScreenElement,
            WorldElement,
        },
        math::{
            ClipUnit,
            LocalUnit,
            Pixel,
        },
        screen::Screen,
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
        Transform(self.0.post_transform(&transform.0))
    }

    pub fn to_native(&self) -> NativeTransform {
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


pub type NativeTransform = [[f32; 4]; 4];


/// Returns what is commonly known as the model matrix
pub fn local_to_world(element: &WorldElement) -> Transform<LocalUnit, Meter> {
    // Can be replaced with `.cast()`, once this PR lands:
    // https://github.com/servo/euclid/pull/440
    let angle = graphics::Angle {
        radians: element.angle.radians as graphics::Scalar,
    };

    graphics::Transform::identity()
        .post_scale(
            element.size.width  as graphics::Scalar,
            element.size.height as graphics::Scalar,
        )
        .post_rotate(angle)
        .post_translate(element.pos.to_vector().cast())
        .into()
}

pub fn local_to_screen(element: &ScreenElement) -> Transform<LocalUnit, Pixel> {
    graphics::Transform::identity()
        .post_scale(element.size.width, element.size.height)
        .post_rotate(element.angle)
        .post_translate(element.pos.to_vector())
        .into()
}

/// Returns what is commonly known as the view matrix
pub fn world_to_screen(camera: &Camera, screen: &Screen)
    -> Transform<Meter, Pixel>
{
    let pixels_per_meter = camera.pixels_per_meter(screen.logical_size());

    graphics::Transform::identity()
        .pre_translate(-camera.center.to_vector().cast::<graphics::Scalar>())
        .post_scale(pixels_per_meter, -pixels_per_meter)
        .post_translate(screen.logical_size().to_vector() / 2.0)
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
        .pre_scale(clip_units_per_pixel.x, -clip_units_per_pixel.y)
        .pre_translate(-screen_size.to_vector() / 2.0)
        .into()
}
