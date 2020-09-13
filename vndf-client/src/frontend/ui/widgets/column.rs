use std::convert::Infallible;

use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            input::Input,
            traits::{
                DrawAt,
                DrawError,
                ProcessInputAt,
                Size,
                Widget,
            },
        },
    },
    graphics,
};


pub struct Column {
    margin:  f32,
    widgets: Vec<Box<dyn Widget>>,
}

impl Column {
    pub fn create(
        margin: f32,
    )
        -> Self
    {
        Self {
            margin,
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: impl Widget + 'static) {
        self.widgets.push(Box::new(widget));
    }

    pub fn add_iter(&mut self,
        widgets: impl IntoIterator<Item=impl Widget + 'static>,
    ) {
        for widget in widgets {
            self.add(widget)
        }
    }

    pub fn widgets_at_mut<F, E>(&mut self, pos: graphics::Pnt2, mut f: F)
        -> Result<(), E>
        where F: FnMut(&mut dyn Widget, graphics::Pnt2) -> Result<(), E>
    {
        let mut next_pos = pos;

        for widget in self.widgets.iter_mut() {
            f(widget.as_mut(), next_pos)?;
            let offset_y = widget.size().height + self.margin;
            next_pos.y += offset_y;
        }

        Ok(())
    }
}

impl Size for Column {
    fn size(&self) -> graphics::Size {
        let mut size = graphics::Size::new(0.0, 0.0);

        for (i, widget) in self.widgets.iter().enumerate() {
            size.width = graphics::Scalar::max(
                size.width,
                widget.size().width,
            );

            size.height += widget.size().height;
            if i < self.widgets.len() - 1 {
                size.height += self.margin;
            }
        }

        size
    }
}

impl ProcessInputAt for Column {
    fn process_input_at(&mut self, input: &mut Input, pos: graphics::Pnt2) {
        self
            .widgets_at_mut::<_, Infallible>(pos, |widget, pos|
                Ok(widget.process_input_at(input, pos))
            )
            .unwrap(); // infallible
    }
}

impl DrawAt for Column {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>
    {
        self.widgets_at_mut(pos, |widget, pos|
            widget.draw_at(res, frame, pos)
        )
    }
}
