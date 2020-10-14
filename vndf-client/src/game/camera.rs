use crate::{
    game::input,
    graphics::{self, screen::Screen, transforms},
    shared::world,
};

pub struct Camera {
    /// The point in the world where the camera is centered
    pub center: world::Pnt2,

    /// The length along the x axis that is currently shown by the camera
    pub view: world::Scalar,

    /// The speed at which the camera is currently moving
    ///
    /// Expressed as a factor that is multiplied with `view` every frame.
    pub speed: world::Scalar,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            center: world::Pnt2::new(0.0, 0.0),
            view: 100_000_000.0, // m
            speed: 1.0,
        }
    }

    pub fn update(
        &mut self,
        dt: world::Scalar,
        own_pos: Option<world::Pnt2>,
        input: &mut input::Handler,
    ) {
        self.update_speed(dt, input);

        self.view *= self.speed;

        if let Some(own_pos) = own_pos {
            self.center = own_pos;
        }
    }

    fn update_speed(&mut self, dt: world::Scalar, input: &mut input::Handler) {
        // Before we do anything, let's slow the speed down a bit. If the user
        // does nothing, we want the camera to slowly stop.
        self.speed = 1.0 + (self.speed - 1.0) * 0.95;

        // Now let's apply the latest input to the speed.
        let input = -input.scroll_acc();
        let input = input as world::Scalar;
        if input * (self.speed - 1.0) >= 0.0 {
            // Current speed and input go into the same direction. Add input to
            // speed.
            self.speed += input * 0.01;
        } else {
            // Input goes into other direction. Stop completely.
            self.speed = 1.0;
        }

        // We want to restrict the camera speed to a maximum value. We do that
        // by choosing the same value as a maximum factor and divisor per
        // second.
        let max_factor = 400.0 as world::Scalar;
        let min_factor = 1.0 / max_factor;

        // So we decided what the maximum factor or divisor for a whole second
        // is going to be, but we'll actually need to deal with a factor/divisor
        // per frame. So next, we'll need to figure out the number of frames per
        // second.
        let frames_per_second = 1.0 / dt;

        // From here, we can figure out the maximum/minimum factors per frame
        // using some n-th roots.
        let max_factor = max_factor.powf(1.0 / frames_per_second);
        let min_factor = min_factor.powf(1.0 / frames_per_second);

        // Now that we've figured out our factors, we can use them to restrict
        // our speed.
        self.speed = world::Scalar::min(self.speed, max_factor);
        self.speed = world::Scalar::max(self.speed, min_factor);
    }

    pub fn world_to_screen(&self, screen: &Screen, point_world: world::Pnt2) -> graphics::Pnt2 {
        transforms::world_to_screen(self, screen)
            .0
            .transform_point(point_world.cast())
    }

    pub fn pixels_per_meter(&self, screen: &Screen) -> graphics::Scalar {
        let world_size_on_screen = self.world_size_on_screen(screen);
        screen.logical_size().width / world_size_on_screen.width as graphics::Scalar
    }

    pub fn world_size_on_screen(&self, screen: &Screen) -> world::Size {
        let screen_size = screen.logical_size();
        let aspect_ratio = screen_size.width / screen_size.height;

        world::Size::new(self.view * aspect_ratio as world::Scalar, self.view)
    }
}
