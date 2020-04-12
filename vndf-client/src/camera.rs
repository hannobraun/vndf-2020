use crate::{
    graphics,
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

    pub fn screen_to_world(&self,
        screen_size:  graphics::Size,
        point_screen: graphics::Pnt2,
    )
        -> world::Pnt2
    {
        let point_screen_origin_centered = point_screen - screen_size / 2.0;

        let world_rect = self.world_size_on_screen(screen_size);
        let point_world = world::Pnt2::new(
            point_screen_origin_centered.x
                * world_rect.width
                / screen_size.width,
            point_screen_origin_centered.y
                * world_rect.height
                / screen_size.height,
        );

        point_world + self.center.to_vector()
    }

    pub fn world_to_screen(&self,
        screen_size: graphics::Size,
        point_world: world::Pnt2,
    )
        -> graphics::Pnt2
    {
        let point_camera = point_world - self.center.to_vector();

        let world_rect = self.world_size_on_screen(screen_size);
        let point_screen_origin_centered = graphics::Pnt2::new(
            point_camera.x * screen_size.width  / world_rect.width,
            point_camera.y * screen_size.height / world_rect.height,
        );

        point_screen_origin_centered + screen_size / 2.0
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
