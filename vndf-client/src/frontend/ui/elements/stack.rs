use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements,
    },
    graphics,
};


pub struct Stack<'a, 'b> {
    margin:   f32,
    elements: &'a mut Vec<Box<dyn StackElement + 'b>>,
}

impl<'a, 'b> Stack<'a, 'b> {
    pub fn new(
        buf:    &'a mut Vec<Box<dyn StackElement + 'b>>,
        margin: f32,
    )
        -> Self
    {
        Self {
            margin,
            elements: buf,
        }
    }

    pub fn add(&mut self, element: impl StackElement + 'b) {
        self.elements.push(Box::new(element));
    }

    pub fn add_iter(&mut self,
        elements: impl IntoIterator<Item=impl StackElement + 'b>,
    ) {
        for element in elements {
            self.add(element)
        }
    }
}

impl<'a, 'b> elements::Draw for Stack<'a, 'b> {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let mut next_pos = pos;

        for element in self.elements.iter_mut() {
            let offset_y = element.size().height + self.margin;
            element.draw(res, frame, next_pos);
            next_pos.y += offset_y;
        }
    }
}


pub trait StackElement: elements::Size + elements::Draw {}

impl<T> StackElement for T where T: elements::Size + elements::Draw {}
