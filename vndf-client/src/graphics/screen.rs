use crate::graphics;


#[derive(Clone, Copy, Debug)]
pub struct Screen {
    /// The physical screen size in pixels
    pub size: graphics::Size,

    /// The scale factor used to convert between physical and logical size
    pub scale_factor: graphics::Scalar,
}

impl Screen {
    pub fn new(size: graphics::Size, scale_factor: graphics::Scalar) -> Self {
        Self {
            size,
            scale_factor,
        }
    }

    pub fn physical_size(&self) -> graphics::Size {
        self.size
    }

    pub fn logical_size(&self) -> graphics::Size {
        self.size / self.scale_factor
    }
}
