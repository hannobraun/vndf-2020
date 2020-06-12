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


pub struct Layout<'r> {
    next_pos: graphics::Pnt2,
    margin:   f32,
    elements: Vec<&'r mut dyn Element>
}

impl<'r> Layout<'r> {
    pub fn new(
        initial_pos: graphics::Pnt2,
        margin:      f32,
    ) -> Self {
        Self {
            next_pos: initial_pos,
            margin,
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, element: &'r mut dyn Element) {
        self.elements.push(element);
    }

    pub fn add_iter(&mut self,
        elements: impl IntoIterator<Item=&'r mut dyn Element>,
    ) {
        for element in elements {
            self.add(element)
        }
    }

    pub fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    ) {
        for element in &mut self.elements {
            let offset_y = element.size().height + self.margin;
            element.draw(res, frame, self.next_pos);
            self.next_pos.y += offset_y;
        }
    }
}
