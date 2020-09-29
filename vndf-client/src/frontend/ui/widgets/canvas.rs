use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::Input,
            traits::{
                Draw,
                DrawAt,
                DrawError,
                ProcessInputAt,
            },
        },
    },
    graphics,
};


pub struct Canvas {
    elements: Vec<(graphics::Pnt2, Box<dyn Element>)>,
}

impl Canvas {
    pub fn create() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_at<E>(&mut self,
        widget:   E,
        position: graphics::Pnt2,
    )
        where E: Element + 'static
    {
        self.elements.push((position, Box::new(widget)));
    }
}

impl Draw for Canvas {
    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
    )
        -> Result<(), DrawError>
    {
        self.draw_at(res, frame, graphics::Pnt2::zero())
    }
}

impl DrawAt for Canvas {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>
    {
        for (element_pos, element) in &mut self.elements {
            let pos = pos + element_pos.to_vector();
            element.draw_at(res, frame, pos)?;
        }

        Ok(())
    }
}

impl ProcessInputAt for Canvas {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        for (widget_pos, element) in &mut self.elements {
            let pos = pos + widget_pos.to_vector();
            element.process_input_at(input, pos);
        }
    }
}


pub trait Element: DrawAt + ProcessInputAt {}

impl<T> Element for T where T: DrawAt + ProcessInputAt {}
