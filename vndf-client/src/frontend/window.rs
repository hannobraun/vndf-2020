use winit::{
    error::OsError,
    event_loop::EventLoop,
    window::{Window as InnerWindow, WindowBuilder},
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
            size.width as graphics::Scalar,
            size.height as graphics::Scalar,
        )
    }

    pub fn scale_factor(&self) -> graphics::Scalar {
        self.0.scale_factor() as graphics::Scalar
    }

    pub fn inner(&self) -> &InnerWindow {
        &self.0
    }
}
