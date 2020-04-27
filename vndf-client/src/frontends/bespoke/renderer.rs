use std::io;

use wgpu_glyph::{
    GlyphBrush,
    GlyphBrushBuilder,
    Section,
};
use winit::event::{
    Event,
    WindowEvent,
};

use crate::{
    game::Game,
    graphics::{
        self,
        elements::{
            ScreenElement,
            WorldElement,
        },
        screen::Screen,
    },
    shared::world::behavior::{
        orbits::Orbit,
        planets::Planet,
        ships::Ship,
    },
};

use super::{
    drawables::Drawables,
    meshes::{
        self,
        Meshes,
    },
    shaders::{
        frag,
        vert,
    },
    window::Window,
};


pub struct Renderer {
    surface:         wgpu::Surface,
    device:          wgpu::Device,
    queue:           wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain:      wgpu::SwapChain,

    glyph_brush: GlyphBrush<'static, ()>,

    drawables: Drawables,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, Error> {
        let surface = wgpu::Surface::create(window.inner());

        let adapter =
            wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference:   wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::all(),
            )
            .await
            .ok_or(Error::AdapterRequest)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                },
            )
            .await;

        let meshes = Meshes::new()
            .map_err(|err| Error::Meshes(err))?;
        let drawables = Drawables::new(&device, &meshes)?;

        let size = window.size();
        let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format:       texture_format,
            width:        size.width  as u32,
            height:       size.height as u32,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(
            &surface,
            &swap_chain_desc,
        );

        let font = include_bytes!("fonts/Tuffy_Bold.ttf");
        let glyph_brush = GlyphBrushBuilder::using_font_bytes(&font[..])
            .map_err(|err| Error::Font(err))?
            .build(&device, texture_format);

        Ok(
            Self {
                surface,
                device,
                queue,
                swap_chain_desc,
                swap_chain,

                glyph_brush,

                drawables,
            }
        )
    }

    pub fn handle_event(&mut self, event: &Event<()>, game: &Game)
        -> Result<(), wgpu::TimeOut>
    {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.swap_chain_desc.width  = size.width;
                self.swap_chain_desc.height = size.height;

                self.swap_chain = self.device.create_swap_chain(
                    &self.surface,
                    &self.swap_chain_desc,
                );
            }
            Event::RedrawRequested(_) => {
                let frame = self.swap_chain.get_next_texture()?;

                let mut encoder = self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None }
                );

                self.draw_background(&frame, &mut encoder);

                for orbit in game.state.active_orbits() {
                    self.draw_orbit(&frame, &mut encoder, &orbit, game);
                }
                for planet in game.state.data.planets.values() {
                    self.draw_planet(&frame, &mut encoder, planet, game);
                }
                for ship in game.state.data.ships.values() {
                    self.draw_ship(&frame, &mut encoder, ship, game);
                }

                let screen_size = self.screen_size();
                self.glyph_brush.queue(Section {
                    text:  "Von Neumann Defense Force",
                    color: [1.0, 1.0, 1.0, 1.0],
                    .. Section::default()
                });
                self.glyph_brush
                    .draw_queued(
                        &self.device,
                        &mut encoder,
                        &frame.view,
                        screen_size.size.width as u32,
                        screen_size.size.height as u32,
                    )
                    // I've checked the code, and it doesn't look like this
                    // actually returns any errors.
                    .unwrap();

                self.queue.submit(&[encoder.finish()]);
            }
            _ => {}
        }

        Ok(())
    }

    fn draw_background(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment:     &frame.view,
                        resolve_target: None,
                        load_op:        wgpu::LoadOp::Clear,
                        store_op:       wgpu::StoreOp::Store,
                        clear_color:    graphics::BACKGROUND_COLOR,
                    },
                ],
                depth_stencil_attachment: None,
            },
        );
    }

    fn draw_orbit(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        orbit:   &Orbit,
        game:    &Game,
    )
        -> Option<()>
    {
        let element = WorldElement::from(orbit);

        let transform = element.transform(
            &game.state.camera,
            self.screen_size().size,
        );

        let pixel_per_m = game.state.camera.pixels_per_meter(
            self.screen_size().size
        );
        let pixel_per_u = [
            pixel_per_m * element.size.width,
            pixel_per_m * element.size.height,
        ];
        let u_per_pixel = [
            1.0 / pixel_per_u[0],
            1.0 / pixel_per_u[1],
        ];

        let orbiter_angle_abs = orbit.orbiter_pos
            .to_vector()
            .angle_from_x_axis();
        let orbiter_angle_to_orbit =
            (orbiter_angle_abs - orbit.arg_of_periapsis).signed();

        let orbiter_dir = orbit.orbiter_pos.to_vector()
            .angle_to(orbit.orbiter_vel)
            .radians;
        let orbiter_dir = if orbiter_dir < 0.0 {
            -1.0
        }
        else if orbiter_dir > 0.0 {
            1.0
        }
        else {
            0.0
        };

        self.drawables.orbit.draw(
            &self.device,
            frame,
            encoder,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::orbit::Uniforms {
                u_per_pixel:   u_per_pixel.into(),
                orbiter_angle: orbiter_angle_to_orbit.radians,
                orbiter_dir,
                .. frag::orbit::Uniforms::default()
            },
        );

        Some(())
    }

    fn draw_planet(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        planet:  &Planet,
        game:    &Game,
    ) {
        let transform = WorldElement::from(planet)
            .transform(&game.state.camera, self.screen_size().size);

        self.drawables.planet.draw(
            &self.device,
            frame,
            encoder,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::planet::Uniforms::default(),
        );
    }

    fn draw_ship(&self,
        frame:   &wgpu::SwapChainOutput,
        encoder: &mut wgpu::CommandEncoder,
        ship:    &Ship,
        game:    &Game,
    )
        -> Option<()>
    {
        let transform = ScreenElement::from_ship(ship, game, self.screen_size().size)?
            .transform(self.screen_size().size);

        self.drawables.ship.draw(
            &self.device,
            frame,
            encoder,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::simple::Uniforms {
                color: ship.color.into(),
            },
        );

        Some(())
    }

    fn screen_size(&self) -> Screen {
        Screen {
            size: graphics::Size::new(
                self.swap_chain_desc.width  as f32,
                self.swap_chain_desc.height as f32,
            ),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Font(wgpu_glyph::rusttype::Error),
    Io(io::Error),
    Meshes(meshes::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
