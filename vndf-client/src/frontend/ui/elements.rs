mod component_stats;
mod frame_time;
mod input_events;
mod instructions;
mod network_stats;
mod panel;
mod stack;
mod text;
mod text_panel;


pub use self::{
    component_stats::ComponentStats,
    frame_time::FrameTime,
    input_events::InputEvents,
    instructions::Instructions,
    network_stats::NetworkStats,
    panel::Panel,
    stack::Stack,
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


pub trait Size {
    fn size(&self) -> graphics::Size;
}

pub trait Draw {
    fn draw(&mut self,
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
