pub mod add_command;
pub mod column;
pub mod commands;
pub mod commands_list;
pub mod component_stats;
pub mod diagnostics;
pub mod frame_time;
pub mod input_events;
pub mod instructions;
pub mod network_stats;
pub mod orbit_info;
pub mod panel;
pub mod scale_factor;
pub mod ship_control;
pub mod ship_info;
pub mod ship_status;
pub mod text;
pub mod text_panel;
pub mod view_size;


pub use self::{
    add_command::AddCommand,
    column::Column,
    commands::Commands,
    commands_list::CommandsList,
    component_stats::ComponentStats,
    diagnostics::Diagnostics,
    frame_time::FrameTime,
    input_events::InputEvents,
    instructions::Instructions,
    network_stats::NetworkStats,
    orbit_info::OrbitInfo,
    panel::Panel,
    scale_factor::ScaleFactor,
    ship_control::ShipControl,
    ship_info::ShipInfo,
    ship_status::ShipStatus,
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

    fn position(mut self,
        anchor: Anchor,
        margin: graphics::Scalar,
        frame:  &Frame,
    )
        -> Positioned<Self>
        where Self: Sized,
    {
        let position = anchor
            .origin(frame)
            .position(&mut self, margin);

        Positioned {
            widget: self,
            position,
        }
    }
}


/// Widgets that track their own position
pub trait Position {
    fn get_pos(&self) -> graphics::Pnt2;
    fn set_pos(&mut self, pos: graphics::Pnt2);
}


/// Widgets that can be drawn without requiring a specific position
pub trait Draw {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    );
}


/// Widgets that can be drawn at a specific position
pub trait DrawAt {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    );
}

impl<T> DrawAt for T where T: Position + Draw {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        self.set_pos(pos);
        self.draw(res, frame);
    }
}


pub struct Positioned<T> {
    pub widget:   T,
    pub position: graphics::Pnt2,
}

impl<T> Positioned<T> {
    pub fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    )
        where T: DrawAt
    {
        self.widget.draw_at(res, frame, self.position)
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
