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
    widgets: Vec<(graphics::Pnt2, Box<dyn Element>)>,
}

impl Canvas {
    pub fn create() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_at(&mut self,
        widget:   Box<dyn Element>,
        position: graphics::Pnt2,
    ) {
        self.widgets.push((position, widget));
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
        for (widget_pos, widget) in &mut self.widgets {
            let pos = pos + widget_pos.to_vector();
            widget.draw_at(res, frame, pos)?;
        }

        Ok(())
    }
}

impl ProcessInputAt for Canvas {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        for (widget_pos, widget) in &mut self.widgets {
            let pos = pos + widget_pos.to_vector();
            widget.process_input_at(input, pos);
        }
    }
}


pub trait Element: DrawAt + ProcessInputAt {}

impl<T> Element for T where T: DrawAt + ProcessInputAt {}
