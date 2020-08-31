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
            },
        },
    },
    graphics,
};


pub struct Column {
    margin:  f32,
    widgets: Vec<Box<dyn Element>>,
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

    pub fn add(&mut self, widget: impl Element + 'static) {
        self.widgets.push(Box::new(widget));
    }

    pub fn add_iter(&mut self,
        widgets: impl IntoIterator<Item=impl Element + 'static>,
    ) {
        for widget in widgets {
            self.add(widget)
        }
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
    fn process_input_at(&mut self, pos: graphics::Pnt2, input: &mut Input) {
        for widget in self.widgets.iter_mut() {
            widget.process_input_at(pos, input);
        }
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
        let mut next_pos = pos;

        for widget in self.widgets.iter_mut() {
            widget.draw_at(res, frame, next_pos)?;
            let offset_y = widget.size().height + self.margin;
            next_pos.y += offset_y;
        }

        Ok(())
    }
}


pub trait Element: Size + ProcessInputAt + DrawAt {}

impl<T> Element for T where T: Size + ProcessInputAt + DrawAt {}
