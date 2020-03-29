use winit::{
    error::OsError,
    event_loop::EventLoop,
    window::{
        Window as InnerWindow,
        WindowBuilder,
    },
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
}
