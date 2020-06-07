use crate::{
    frontend::drawers::{
        DrawResources,
        Frame,
    },
    graphics,
    ui,
};

use super::element;


pub struct Layout<'r> {
    res:      &'r mut DrawResources,
    frame:    &'r mut Frame,
    next_pos: graphics::Pnt2,
    margin:   f32,
}

impl<'r> Layout<'r> {
    pub fn new(
        res:         &'r mut DrawResources,
        frame:       &'r mut Frame,
        initial_pos: graphics::Pnt2,
        margin:      f32,
    ) -> Self {
        Self {
            res,
            frame,
            next_pos: initial_pos,
            margin,
        }
    }

    pub fn draw(&mut self, element: &'r ui::Element) {
        let size = element::draw(
            self.res,
            self.frame,
            self.next_pos,
            element.text.as_str(),
        );

        self.next_pos.y += size.height + self.margin;
    }
}
