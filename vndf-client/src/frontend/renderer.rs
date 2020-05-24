use std::io;

use log::{
    debug,
    warn,
};
use winit::event::{
    Event,
    WindowEvent,
};

use crate::{
    Graphics,
    UiOption,
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
        explosions::Explosion,
        orbits::Orbit,
        planets::Planet,
        ships::Ship,
    },
};

use super::{
    drawables::Drawables,
    drawers::{
        DrawResources,
        Frame,
        draw_background,
    },
    meshes::{
        self,
        Meshes,
    },
    shaders::{
        frag,
        vert,
    },
    ui::{
        self,
        Ui,
    },
    window::Window,
};


pub struct Renderer {
    surface:         wgpu::Surface,
    queue:           wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain:      wgpu::SwapChain,
    draw_res:        DrawResources,

    ui: Box<dyn Ui>,

    scale_factor: f32,
}

impl Renderer {
    pub async fn new(window: &Window, graphics: Graphics, ui: UiOption)
        -> Result<Self, Error>
    {
        let backend = select_backend(graphics);
        debug!("Backend selected: {:?}", backend);

        let surface = wgpu::Surface::create(window.inner());

        let adapter =
            wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference:   wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                backend,
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
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(
            &surface,
            &swap_chain_desc,
        );

        let scale_factor = window.scale_factor();

        let ui: Box<dyn Ui> = match ui {
            UiOption::Basic => {
                Box::new(
                    ui::Basic::new(&device, texture_format)
                        .map_err(|err| Error::Font(err))?
                )
            }
            UiOption::Conrod => {
                let screen = screen(&swap_chain_desc, scale_factor);

                Box::new(
                    ui::Conrod::new(&device, texture_format, &screen)
                        .map_err(|err| Error::Font(err))?
                )
            }
        };

        let draw_res = DrawResources {
            device,
            drawables,
        };

        Ok(
            Self {
                surface,
                queue,
                swap_chain_desc,
                swap_chain,
                draw_res,

                ui,

                scale_factor,
            }
        )
    }

    pub fn handle_event(&mut self, event: &Event<()>, game: &Game)
        -> Result<(), Error>
    {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.swap_chain_desc.width  = size.width;
                self.swap_chain_desc.height = size.height;

                self.swap_chain = self.draw_res.device.create_swap_chain(
                    &self.surface,
                    &self.swap_chain_desc,
                );
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    ..
                },
                ..
            } => {
                self.scale_factor = *scale_factor as f32;
            }
            Event::RedrawRequested(_) => {
                let mut frame = Frame {
                    screen: self.screen(),
                    output: self.swap_chain.get_next_texture()
                        .map_err(|_| Error::TimeOut)?,
                    encoder: self.draw_res.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor { label: None }
                    ),
                };

                draw_background(&mut frame);

                for orbit in game.state.active_orbits() {
                    Self::draw_orbit(
                        &self.draw_res,
                        &mut frame,
                        &orbit,
                        game,
                    );
                }
                for planet in game.state.data.planets.values() {
                    self.draw_planet(
                        &mut frame,
                        planet,
                        game,
                    );
                }
                for ship in game.state.data.ships.values() {
                    self.draw_ship(
                        &mut frame,
                        ship,
                        game,
                    );
                }
                for explosion in game.state.data.explosions.values() {
                    self.draw_explosion(
                        &mut frame,
                        explosion,
                        game,
                    );
                }

                self.ui
                    .draw(
                        &self.draw_res.device,
                        &mut frame,
                        &mut self.draw_res.drawables,
                        game,
                    )
                    .map_err(|()| Error::Ui)?;

                self.queue.submit(&[frame.encoder.finish()]);
            }
            _ => {}
        }

        self.ui.handle_event(event, &self.screen());

        Ok(())
    }

    fn draw_orbit(
        res:   &DrawResources,
        frame: &mut Frame,
        orbit: &Orbit,
        game:  &Game,
    )
        -> Option<()>
    {
        let element = WorldElement::from(orbit);

        let transform = element.transform(
            &game.state.camera,
            frame.screen.size,
        );

        let pixel_per_m = game.state.camera.pixels_per_meter(
            frame.screen.size
        );
        let pixel_per_u = [
            pixel_per_m * element.size.width,
            pixel_per_m * element.size.height,
        ];
        let u_per_pixel = [
            1.0 / pixel_per_u[0],
            1.0 / pixel_per_u[1],
        ];

        let orbiter_angle_abs = orbit.orbiter.pos
            .to_vector()
            .angle_from_x_axis();
        let orbiter_angle_to_orbit =
            (orbiter_angle_abs - orbit.arg_of_periapsis).signed();

        let orbiter_dir = orbit.orbiter.pos.to_vector()
            .angle_to(orbit.orbiter.vel)
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

        res.drawables.orbit.draw(
            &res.device,
            frame,
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
        frame:  &mut Frame,
        planet: &Planet,
        game:   &Game,
    ) {
        let transform = WorldElement::from(planet)
            .transform(&game.state.camera, frame.screen.size);

        self.draw_res.drawables.planet.draw(
            &self.draw_res.device,
            frame,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::planet::Uniforms::default(),
        );
    }

    fn draw_ship(&self,
        frame: &mut Frame,
        ship:  &Ship,
        game:  &Game,
    )
        -> Option<()>
    {
        let transform = ScreenElement::from_ship(ship, game, &frame.screen)?
            .transform(frame.screen.size);

        self.draw_res.drawables.ship.draw(
            &self.draw_res.device,
            frame,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::simple::Uniforms {
                color: ship.color.into(),
            },
        );

        Some(())
    }

    fn draw_explosion(&self,
        frame:     &mut Frame,
        explosion: &Explosion,
        game:      &Game,
    )
        -> Option<()>
    {
        let transform =
            ScreenElement::from_explosion(
                explosion,
                game,
                &frame.screen,
            )?
            .transform(frame.screen.size);

        self.draw_res.drawables.explosion.draw(
            &self.draw_res.device,
            frame,
            vert::simple::Uniforms {
                transform: transform.into(),
            },
            frag::explosion::Uniforms {
                strength_total: explosion.strength_total,
                strength_left:  explosion.strength_left,
            },
        );

        Some(())
    }

    fn screen(&self) -> Screen {
        screen(&self.swap_chain_desc, self.scale_factor)
    }
}


fn select_backend(graphics: Graphics) -> wgpu::BackendBit {
    match graphics {
        Graphics::Auto => {
            debug!("Automatically selecting backend based on target platform");

            if cfg!(target_os = "linux") {
                return wgpu::BackendBit::VULKAN;
            }
            if cfg!(target_os = "windows") {
                return wgpu::BackendBit::DX12;
            }

            warn!("Platform not recognized; leaving backend selection to wgpu");
            wgpu::BackendBit::PRIMARY
        }

        Graphics::DirectX11 => wgpu::BackendBit::DX11,
        Graphics::DirectX12 => wgpu::BackendBit::DX12,
        Graphics::Metal     => wgpu::BackendBit::METAL,
        Graphics::OpenGl    => wgpu::BackendBit::GL,
        Graphics::Vulkan    => wgpu::BackendBit::VULKAN,
        Graphics::WebGpu    => wgpu::BackendBit::BROWSER_WEBGPU,
    }
}

fn screen(swap_chain_desc: &wgpu::SwapChainDescriptor, scale_factor: f32)
    -> Screen
{
    Screen {
        size: graphics::Size::new(
            swap_chain_desc.width  as f32,
            swap_chain_desc.height as f32,
        ),
        scale_factor,
    }
}


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Font(wgpu_glyph::rusttype::Error),
    Io(io::Error),
    Meshes(meshes::Error),
    TimeOut,
    Ui,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
