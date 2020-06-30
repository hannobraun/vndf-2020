use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::widgets::Widget,
    },
    graphics,
};


pub struct Stack<'a> {
    margin:  f32,
    widgets: &'a mut Vec<Box<dyn Widget>>,
}

impl<'a> Stack<'a> {
    pub fn new(
        buf:    &'a mut Vec<Box<dyn Widget>>,
        margin: f32,
    )
        -> Self
    {
        Self {
            margin,
            widgets: buf,
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
}

impl Widget for Stack<'_> {
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

    fn draw(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let mut next_pos = pos;

        for widget in self.widgets.iter_mut() {
            widget.draw(res, frame, next_pos);
            let offset_y = widget.size().height + self.margin;
            next_pos.y += offset_y;
        }
    }
}
