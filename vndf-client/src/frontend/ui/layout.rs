use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::Element,
    },
    graphics,
};


pub struct Layout {
    next_pos: graphics::Pnt2,
    margin:   f32,
}

impl Layout {
    pub fn new(
        initial_pos: graphics::Pnt2,
        margin:      f32,
    ) -> Self {
        Self {
            next_pos: initial_pos,
            margin,
        }
    }

    pub fn draw(&mut self,
        res:     &mut DrawResources,
        frame:   &mut Frame,
        element: impl Element,
    ) {
        self.draw_iter(res, frame, Some(element))
    }

    pub fn draw_iter(&mut self,
        res:      &mut DrawResources,
        frame:    &mut Frame,
        elements: impl IntoIterator<Item=impl Element>,
    ) {
        for mut element in elements {
            let offset_y = element.size().height + self.margin;
            element.draw(res, frame, self.next_pos);
            self.next_pos.y += offset_y;
        }
    }
}
