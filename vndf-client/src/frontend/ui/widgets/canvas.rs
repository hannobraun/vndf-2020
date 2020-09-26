use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::traits::{
            Draw,
            DrawAt,
            DrawError,
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


pub trait AddAtWidget: DrawAt {}

impl<T> AddAtWidget for T where T: DrawAt {}
