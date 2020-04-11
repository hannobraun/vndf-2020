use crate::{
    game::coords::{
        Screen,
        World,
    },
    shared::world::math::{
        Pnt2,
        Vec2,
    },
};


pub struct Camera {
    pub center: World<Pnt2>,
    pub zoom:   f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: World(Pnt2::new(0.0, 0.0)),
            zoom:   1.0,
        }
    }

    pub fn screen_to_world(&self,
        screen_size:  Screen<Vec2>,
        point_screen: Screen<Pnt2>,
    )
        -> World<Pnt2>
    {
        let point_screen_origin_centered = point_screen - screen_size / 2.0;

        let world_rect = self.world_size_on_screen(screen_size);
        let point_world = World(
            Pnt2::new(
                point_screen_origin_centered.0.x
                    * world_rect.0.x
                    / screen_size.0.x,
                point_screen_origin_centered.0.y
                    * world_rect.0.y
                    / screen_size.0.y,
            )
        );

        point_world + self.center.to_vec()
    }

    pub fn world_to_screen(&self,
        screen_size: Screen<Vec2>,
        point_world: impl Into<World<Pnt2>>,
    )
        -> Screen<Pnt2>
    {
        let point_world = point_world.into();

        let point_camera = point_world - self.center.to_vec();

        let world_rect = self.world_size_on_screen(screen_size);
        let point_screen_origin_centered = Screen(
            Pnt2::new(
                point_camera.0.x * screen_size.0.x / world_rect.0.x,
                point_camera.0.y * screen_size.0.y / world_rect.0.y,
            )
        );

        point_screen_origin_centered + screen_size / 2.0
    }

    pub fn pixels_per_unit(&self, screen_size: Screen<Vec2>) -> f32 {
        let world_size_on_screen = self.world_size_on_screen(screen_size);
        screen_size.0.x / world_size_on_screen.0.x
    }

    pub fn world_size_on_screen(&self, screen_size: Screen<Vec2>)
        -> World<Vec2>
    {
        let aspect_ratio = screen_size.0.x / screen_size.0.y;

        let min_world_size_on_screen = 100_000_000.0; // m

        let default_world_size_on_screen = World(
            if aspect_ratio >= 1.0 {
                Vec2::new(
                    min_world_size_on_screen * aspect_ratio,
                    min_world_size_on_screen,
                )
            }
            else {
                Vec2::new(
                    min_world_size_on_screen,
                    min_world_size_on_screen / aspect_ratio,
                )
            }
        );

        default_world_size_on_screen / self.zoom
    }
}
