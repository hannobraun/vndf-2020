mod diagnostics;
mod frame_time;
mod instructions;
mod panel;
mod text;
mod text_panel;


pub use self::{
    diagnostics::Diagnostics,
    frame_time::FrameTime,
    instructions::Instructions,
    panel::Panel,
    text::Text,
    text_panel::TextPanel,
};


use std::fmt;

use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};

use self::text::NoBoundsError;


pub trait Element {
    fn size(&self) -> graphics::Size;
    fn draw(self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    );
}


#[derive(Debug)]
pub enum TextPanelRelatedError {
    Fmt(fmt::Error),
    NoBounds(NoBoundsError),
}

impl From<fmt::Error> for TextPanelRelatedError {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl From<NoBoundsError> for TextPanelRelatedError {
    fn from(err: NoBoundsError) -> Self {
        Self::NoBounds(err)
    }
}
