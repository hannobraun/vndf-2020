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


pub struct Stack<'r> {
    margin:   f32,
    elements: Vec<&'r mut dyn StackElement>
}

impl<'r> Stack<'r> {
    pub fn new(margin: f32) -> Self {
        Self {
            margin,
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, element: &'r mut dyn StackElement) {
        self.elements.push(element);
    }

    pub fn add_iter(&mut self,
        elements: impl IntoIterator<Item=&'r mut dyn StackElement>,
    ) {
        for element in elements {
            self.add(element)
        }
    }
}

impl<'r> elements::Draw for Stack<'r> {
    fn draw(&mut self,
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


pub trait StackElement: elements::Size + elements::Draw {}

impl<T> StackElement for T where T: elements::Size + elements::Draw {}
