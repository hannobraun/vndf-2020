use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        ui::{
            anchor::{
                Anchor,
                Horizontal,
                Vertical,
            },
            input::Input,
            traits::{
                Draw,
                DrawAt,
                DrawError,
                ProcessInputAt,
                Size,
            },
        },
    },
    graphics,
};


pub struct Canvas {
    elements: Vec<(graphics::Pnt2, Box<dyn Element>)>,
    margin:  f32,
}

impl Canvas {
    pub fn create(margin: f32) -> Self {
        Self {
            elements: Vec::new(),
            margin,
        }
    }

    pub fn add_at<E>(&mut self,
        element:  E,
        position: graphics::Pnt2,
    )
        where E: Element + 'static
    {
        self.elements.push((position, Box::new(element)));
    }

    pub fn add_anchored<E>(&mut self,
        element: E,
        anchor:  Anchor,
        frame:   &Frame,
    )
        where E: Element + Size + 'static
    {
        let size = frame.screen.logical_size();

        let x = match anchor.horizontal {
            Horizontal::Left  => 0.0,
            Horizontal::Right => size.width,
        };
        let y = match anchor.vertical {
            Vertical::Top    => 0.0,
            Vertical::Bottom => size.height,
        };

        let offset_x = match anchor.horizontal {
            Horizontal::Left  => self.margin,
            Horizontal::Right => -element.size().width - self.margin
        };
        let offset_y = match anchor.vertical {
            Vertical::Top    => self.margin,
            Vertical::Bottom => -element.size().height - self.margin
        };

        let position = graphics::Pnt2::new(x + offset_x, y + offset_y);

        self.elements.push((position, Box::new(element)));
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
        for (element_pos, element) in &mut self.elements {
            let pos = pos + element_pos.to_vector();
            element.process_input_at(input, pos);
        }
    }
}


pub trait Element: DrawAt + ProcessInputAt {}

impl<T> Element for T where T: DrawAt + ProcessInputAt {}
