use crate::{
    frontend::{
        drawables,
        drawers::{
            DrawResources,
            Frame,
        },
    },
    graphics,
};

use super::{
    anchor::{
        self,
        Anchor,
    },
    input::Input,
};


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


/// Widgets that process input
pub trait ProcessInputAt {
    fn process_input_at(&mut self, pos: graphics::Pnt2, input: &mut Input);
}


/// Widgets that can be drawn without requiring a specific position
pub trait Draw {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    )
        -> Result<(), DrawError>;
}


/// Widgets that can be drawn at a specific position
pub trait DrawAt {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>;
}

impl<T> DrawAt for T where T: Position + Draw {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>
    {
        self.set_pos(pos);
        self.draw(res, frame)?;

        Ok(())
    }
}


#[derive(Debug)]
pub struct DrawError(drawables::text::Error);

impl From<drawables::text::Error> for DrawError {
    fn from(err: drawables::text::Error) -> Self {
        Self(err)
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
        -> Result<(), DrawError>
        where T: DrawAt
    {
        self.widget.draw_at(res, frame, self.position)
    }
}
