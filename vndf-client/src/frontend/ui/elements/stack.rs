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


pub struct Stack<'a, 'b> {
    margin:   f32,
    elements: &'a mut Vec<Box<dyn Element + 'b>>,
}

impl<'a, 'b> Stack<'a, 'b> {
    pub fn new(
        buf:    &'a mut Vec<Box<dyn Element + 'b>>,
        margin: f32,
    )
        -> Self
    {
        Self {
            margin,
            elements: buf,
        }
    }

    pub fn add(&mut self, element: impl Element + 'b) {
        self.elements.push(Box::new(element));
    }

    pub fn add_iter(&mut self,
        elements: impl IntoIterator<Item=impl Element + 'b>,
    ) {
        for element in elements {
            self.add(element)
        }
    }
}

impl Element for Stack<'_, '_> {
    fn size(&self) -> graphics::Size {
        let mut size = graphics::Size::new(0.0, 0.0);

        for (i, element) in self.elements.iter().enumerate() {
            size.width = graphics::Scalar::max(
                size.width,
                element.size().width,
            );

            size.height += element.size().height;
            if i < self.elements.len() - 1 {
                size.height += self.margin;
            }
        }

        size
    }

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let mut next_pos = pos;

        for element in self.elements.iter_mut() {
            element.draw(res, frame, next_pos);
            let offset_y = element.size().height + self.margin;
            next_pos.y += offset_y;
        }
    }
}
