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
        element: &mut dyn Element,
    ) {
        let offset_y = element.size().height + self.margin;
        element.draw(res, frame, self.next_pos);
        self.next_pos.y += offset_y;
    }

    pub fn draw_iter<'r>(&mut self,
        res:      &mut DrawResources,
        frame:    &mut Frame,
        elements: impl IntoIterator<Item=&'r mut dyn Element>,
    ) {
        for element in elements {
            self.draw(res, frame, element)
        }
    }
}
