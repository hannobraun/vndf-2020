use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::traits::{
            Draw,
            DrawError,
            Widget,
        },
    },
    graphics,
};


pub struct Canvas {
    widgets: Vec<(graphics::Pnt2, Box<dyn Widget>)>,
}

impl Canvas {
    pub fn create() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, position: graphics::Pnt2, widget: Box<dyn Widget>) {
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
