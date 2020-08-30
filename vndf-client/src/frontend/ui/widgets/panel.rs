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
    pub fn create(size: graphics::Size) -> Self {
        Self {
            size,
            color: None,
        }
    }

    pub fn color(&mut self, color: [f32; 4]) {
        self.color = Some(color);
    }
}

impl Size for Panel {
    fn size(&self) -> graphics::Size {
        self.size
    }
}

impl ProcessInputAt for Panel {
    fn process_input_at(&mut self, _: graphics::Pnt2, _: &mut Input) {}
}

impl DrawAt for Panel {
    fn draw_at(&mut self,
        res:   &mut DrawResources,
        frame: &mut Frame,
        pos:   graphics::Pnt2,
    )
        -> Result<(), DrawError>
    {
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

        Ok(())
    }
}
