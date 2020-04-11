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
        screen_size:  graphics::Vec2,
        point_screen: graphics::Pnt2,
    )
        -> world::Pnt2
    {
        let point_screen_origin_centered = point_screen - screen_size / 2.0;

        let world_rect = self.world_size_on_screen(screen_size);
        let point_world = world::Pnt2::new(
            point_screen_origin_centered.x
                * world_rect.x
                / screen_size.x,
            point_screen_origin_centered.y
                * world_rect.y
                / screen_size.y,
        );

        point_world + self.center.to_vector()
    }

    pub fn world_to_screen(&self,
        screen_size: graphics::Vec2,
        point_world: world::Pnt2,
    )
        -> graphics::Pnt2
    {
        let point_camera = point_world - self.center.to_vector();

        let world_rect = self.world_size_on_screen(screen_size);
        let point_screen_origin_centered = graphics::Pnt2::new(
            point_camera.x * screen_size.x / world_rect.x,
            point_camera.y * screen_size.y / world_rect.y,
        );

        point_screen_origin_centered + screen_size / 2.0
    }

    pub fn pixels_per_unit(&self, screen_size: graphics::Vec2) -> f32 {
        let world_size_on_screen = self.world_size_on_screen(screen_size);
        screen_size.x / world_size_on_screen.x
    }

    pub fn world_size_on_screen(&self, screen_size: graphics::Vec2)
        -> world::Vec2
    {
        let aspect_ratio = screen_size.x / screen_size.y;

        let min_world_size_on_screen = 100_000_000.0; // m

        let default_world_size_on_screen = if aspect_ratio >= 1.0 {
            world::Vec2::new(
                min_world_size_on_screen * aspect_ratio,
                min_world_size_on_screen,
            )
        }
        else {
            world::Vec2::new(
                min_world_size_on_screen,
                min_world_size_on_screen / aspect_ratio,
            )
        };

        default_world_size_on_screen / self.zoom
    }
}
