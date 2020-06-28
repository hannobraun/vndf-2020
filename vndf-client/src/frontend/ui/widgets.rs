pub mod component_stats;
pub mod diagnostics;
pub mod frame_time;
pub mod input_events;
pub mod instructions;
pub mod network_stats;
pub mod panel;
pub mod scale_factor;
pub mod ship_control;
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
    ship_control::ShipControl,
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


pub trait Widget {
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

    fn position(&mut self,
        anchor: Anchor,
        margin: graphics::Scalar,
        frame:  &Frame,
    )
        -> Positioned
        where Self: Sized,
    {
        let position = anchor
            .origin(frame)
            .position(self, margin);

        Positioned {
            widget: self,
            position,
        }
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    );
}


pub struct Positioned<'r> {
    pub widget:   &'r mut dyn Widget,
    pub position: graphics::Pnt2,
}

impl Positioned<'_> {
    pub fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    ) {
        self.widget.draw(res, frame, self.position)
    }
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
