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
    margin:   f32,
    elements: Vec<&'r mut dyn Element>
}

impl<'r> Layout<'r> {
    pub fn new(margin: f32) -> Self {
        Self {
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
        pos:   graphics::Pnt2,
    ) {
        let mut next_pos = pos;

        for element in &mut self.elements {
            let offset_y = element.size().height + self.margin;
            element.draw(res, frame, next_pos);
            next_pos.y += offset_y;
        }
    }
}
