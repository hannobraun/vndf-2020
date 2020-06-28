use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::elements::Widget,
    },
    graphics,
};


pub struct Stack<'a, 'b> {
    margin:  f32,
    widgets: &'a mut Vec<Box<dyn Widget + 'b>>,
}

impl<'a, 'b> Stack<'a, 'b> {
    pub fn new(
        buf:    &'a mut Vec<Box<dyn Widget + 'b>>,
        margin: f32,
    )
        -> Self
    {
        Self {
            margin,
            widgets: buf,
        }
    }

    pub fn add(&mut self, widget: impl Widget + 'b) {
        self.widgets.push(Box::new(widget));
    }

    pub fn add_iter(&mut self,
        widgets: impl IntoIterator<Item=impl Widget + 'b>,
    ) {
        for widget in widgets {
            self.add(widget)
        }
    }
}

impl Widget for Stack<'_, '_> {
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
