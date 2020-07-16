use crate::{
    frontend::{
        drawers::{
            DrawResources,
            Frame,
        },
        shaders::{
            frag,
            vert,
        },
        ui::widgets::{
            DrawAt,
            Size,
        },
    },
    graphics::{
        self,
        elements::ScreenElement,
    },
};


pub struct Panel {
    size:  graphics::Size,
    color: Option<[f32; 4]>,
}

impl Panel {
    pub fn new(size: graphics::Size) -> Self {
        Self {
            size,
            color: None,
        }
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = Some(color);
        self
    }
}

impl Size for Panel {
    fn size(&self) -> graphics::Size {
        self.size
    }
}

impl DrawAt for Panel {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    ) {
        let element = ScreenElement {
            size: self.size,
            pos,
            angle: graphics::Angle::zero(),
        };
        let transform = element
            .transform(&frame.screen)
            .into();
        let color = self.color.unwrap_or([0.0, 0.0, 0.0, 0.95]);

        res.drawables.square.draw(
            &res.device,
            frame,
            vert::simple::Uniforms {
                transform,
            },
            frag::simple::Uniforms {
                color: color.into(),
            },
        );
    }
}
