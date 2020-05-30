use std::iter;

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


const R: f32 = 0.3;
const G: f32 = 0.3;
const B: f32 = 1.0;


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

    let max_screen_len = f32::max(
        world_size_on_screen.x,
        world_size_on_screen.y,
    );
    let mut cell_size = (2.0f32).powf(max_screen_len.log2().ceil());
    let mut alpha     = 1.0;

    loop {
        draw_cells(
            res,
            frame,
            start,
            end,
            cell_size,
            alpha,
            camera,
        );

        cell_size /= 2.0;

        let screen_to_cell = max_screen_len / cell_size;

        const ALPHA_LIMIT: f32 = 2.0;
        const LOWER_LIMIT: f32 = 8.0;

        if screen_to_cell > ALPHA_LIMIT {
            alpha = 1.0
                - (screen_to_cell - ALPHA_LIMIT) / (LOWER_LIMIT - ALPHA_LIMIT);
        }
        if screen_to_cell > LOWER_LIMIT {
            break;
        }
    }
}

fn draw_cells(
    res:       &mut DrawResources,
    frame:     &mut Frame,
    start:     world::Pnt2,
    end:       world::Pnt2,
    cell_size: f32,
    alpha:     f32,
    camera:    &Camera,
) {
    for x in iter(start.x, end.x, cell_size) {
        draw_line(
            res,
            frame,
            world::Pnt2::new(x, start.y),
            world::Pnt2::new(x, end.y),
            alpha,
            camera,
        );
    }

    for y in iter(start.y, end.y, cell_size) {
        draw_line(
            res,
            frame,
            world::Pnt2::new(start.x, y),
            world::Pnt2::new(end.x,   y),
            alpha,
            camera,
        );
    }
}

fn draw_line(
    res:    &mut DrawResources,
    frame:  &mut Frame,
    start:  world::Pnt2,
    end:    world::Pnt2,
    alpha:  f32,
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
            color: [R, G, B, alpha].into(),
        }
    );
}

fn iter(start: f32, end: f32, cell_size: f32) -> impl Iterator<Item=f32> {
    let start = start - start % cell_size;

    let mut i = 0;

    iter::from_fn(move || {
        let v = start + cell_size * i as f32;
        i += 1;

        if v <= end {
            Some(v)
        }
        else {
            None
        }
    })
}
