use winit::{
    error::OsError,
    event::Event,
    event_loop::EventLoop,
    window::{
        Window as InnerWindow,
        WindowBuilder,
    },
};

use crate::graphics;


pub struct Window(InnerWindow);

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

    pub fn size(&self) -> graphics::Size {
        let size = self.0.inner_size();

        graphics::Size::new(
            size.width  as f32,
            size.height as f32,
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

    pub fn inner(&self) -> &InnerWindow {
        &self.0
    }
}
