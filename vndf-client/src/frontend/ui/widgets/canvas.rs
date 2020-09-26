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
    widgets: Vec<(graphics::Pnt2, Box<dyn AddAtWidget>)>,
}

impl Canvas {
    pub fn create() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_at(&mut self,
        widget:   Box<dyn AddAtWidget>,
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
        for (widget_pos, widget) in &mut self.widgets {
            widget.draw_at(res, frame, *widget_pos)?;
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


pub trait AddAtWidget: DrawAt + ProcessInputAt {}

impl<T> AddAtWidget for T where T: DrawAt + ProcessInputAt {}
