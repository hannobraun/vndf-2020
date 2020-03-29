use winit::{
    error::OsError,
    event::Event,
    event_loop::EventLoop,
    window::{
        Window as InnerWindow,
        WindowBuilder,
    },
};

use crate::{
    game::coords::Screen,
    shared::math::Vec2,
};


pub struct Window(pub InnerWindow);

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, OsError> {
        let inner = WindowBuilder::new()
            .with_title("Von Neumann Defense Force")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)?;

        Ok(Self(inner))
    }

    pub fn size(&self) -> Screen<Vec2> {
        let size = self.0.inner_size();

        Screen(
            Vec2::new(
                size.width  as f32,
                size.height as f32,
            )
        )
    }

    pub fn handle_event(&self, event: &Event<()>) {
        match event {
            Event::MainEventsCleared => {
                self.0.request_redraw()
            }

            _ => (),
        }
    }
}
