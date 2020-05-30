use crate::{
    graphics::{
        self,
        transforms,
    },
    shared::world,
};


pub struct Camera {
    /// The point in the world where the camera is centered
    pub center: world::Pnt2,

    /// The length along the x axis that is currently shown by the camera
    pub view: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: world::Pnt2::new(0.0, 0.0),
            view:   100_000_000.0, // m
        }
    }

    pub fn world_to_screen(&self,
        screen_size: graphics::Size,
        point_world: world::Pnt2,
    )
        -> graphics::Pnt2
    {
        transforms::world_to_screen(self, screen_size).0
            .transform_point(point_world)
    }

    pub fn pixels_per_meter(&self, screen_size: graphics::Size) -> f32 {
        let world_size_on_screen = self.world_size_on_screen(screen_size);
        screen_size.width / world_size_on_screen.width
    }

    pub fn world_size_on_screen(&self, screen_size: graphics::Size)
        -> world::Size
    {
        let aspect_ratio = screen_size.width / screen_size.height;

        world::Size::new(
            self.view * aspect_ratio,
            self.view,
        )
    }
}
