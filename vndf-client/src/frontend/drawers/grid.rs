use crate::{
    frontend::shaders::{
        frag,
        vert,
    },
    game::{
        Game,
        camera::Camera,
    },
    graphics::{
        self,
        elements::ScreenElement,
    },
    shared::world,
};

use super::{
    DrawResources,
    Frame,
};


pub fn draw_grid(
    res:   &mut DrawResources,
    frame: &mut Frame,
    game:  &Game,
) {
    let camera = &game.state.camera;
    let world_size_on_screen = camera
        .world_size_on_screen(frame.screen.size)
        .to_vector();

    let start = camera.center - world_size_on_screen / 2.0;
    let end   = start + world_size_on_screen;

    let cell_size = 33_554_432.0;

    draw_cells(
        res,
        frame,
        start,
        end,
        cell_size,
        camera,
    );
}

fn draw_cells(
    res:       &mut DrawResources,
    frame:     &mut Frame,
    start:     world::Pnt2,
    end:       world::Pnt2,
    cell_size: f32,
    camera:    &Camera,
) {
    let mut x = start.x - start.x % cell_size;
    loop {
        draw_line(
            res,
            frame,
            world::Pnt2::new(x, start.y),
            world::Pnt2::new(x, end.y),
            camera,
        );

        x += cell_size;

        if x > end.x {
            break;
        }
    }

    let mut y = start.y - start.y % cell_size;
    loop {
        draw_line(
            res,
            frame,
            world::Pnt2::new(start.x,  y),
            world::Pnt2::new(end.x,    y),
            camera,
        );

        y += cell_size;

        if y > end.y {
            break;
        }
    }
}

fn draw_line(
    res:    &mut DrawResources,
    frame:  &mut Frame,
    start:  world::Pnt2,
    end:    world::Pnt2,
    camera: &Camera,
) {
    let start = camera.world_to_screen(frame.screen.size, start);
    let end   = camera.world_to_screen(frame.screen.size, end);

    let start_to_end = end - start;

    let length    = start_to_end.length();
    let thickness = 2.0;

    let transform =
        ScreenElement {
            size:  graphics::Size::new(length, thickness),
            pos:   start,
            angle: -start_to_end.angle_from_x_axis(),
        }
        .transform(frame.screen.size);

    res.drawables.square.draw(
        &res.device,
        frame,
        vert::simple::Uniforms {
            transform: transform.into(),
        },
        frag::simple::Uniforms {
            color: [0.3, 0.3, 1.0, 1.0].into(),
        }
    );
}
