pub mod component_stats;
pub mod diagnostics;
pub mod frame_time;
pub mod input_events;
pub mod instructions;
pub mod network_stats;
pub mod panel;
pub mod scale_factor;
pub mod ship_status;
pub mod stack;
pub mod text;
pub mod text_panel;
pub mod view_size;


pub use self::{
    component_stats::ComponentStats,
    diagnostics::Diagnostics,
    frame_time::FrameTime,
    input_events::InputEvents,
    instructions::Instructions,
    network_stats::NetworkStats,
    panel::Panel,
    scale_factor::ScaleFactor,
    ship_status::ShipStatus,
    stack::Stack,
    text::Text,
    text_panel::TextPanel,
    view_size::ViewSize,
};


use std::fmt;

use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
};

use super::anchor::{
    self,
    Anchor,
};

use self::text::NoBoundsError;


pub trait Size {
    fn size(&self) -> graphics::Size;

    fn offset(&self, anchor: Anchor, margin: graphics::Scalar)
        -> graphics::Vec2
    {
        let x = match anchor.horizontal {
            anchor::Horizontal::Left  => margin,
            anchor::Horizontal::Right => -self.size().width - margin
        };
        let y = match anchor.vertical {
            anchor::Vertical::Top    => margin,
            anchor::Vertical::Bottom => -self.size().height - margin
        };

        graphics::Vec2::new(x, y)
    }
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
