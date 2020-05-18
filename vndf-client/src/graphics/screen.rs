use crate::graphics;


#[derive(Debug)]
pub struct Screen {
    /// The physical screen size in pixels
    pub size: graphics::Size,

    /// The scale factor used to convert between physical and logical size
    pub scale_factor: f32,
}

impl Screen {
    pub fn logical_size(&self) -> graphics::Size {
        self.size / self.scale_factor
    }
}
