use crate::{
    graphics::{
        self,
        transforms,
    },
    shared::world,
};


pub struct Camera {
    pub center: world::Pnt2,
    pub zoom:   f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: world::Pnt2::new(0.0, 0.0),
            zoom:   1.0,
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

        let min_world_size_on_screen = 100_000_000.0; // m

        let default_world_size_on_screen = if aspect_ratio >= 1.0 {
            world::Size::new(
                min_world_size_on_screen * aspect_ratio,
                min_world_size_on_screen,
            )
        }
        else {
            world::Size::new(
                min_world_size_on_screen,
                min_world_size_on_screen / aspect_ratio,
            )
        };

        default_world_size_on_screen / self.zoom
    }
}
