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
        screen::Screen,
    },
};

use super::{
    drawables::{
        self,
        Drawables,
    },
    drawers::{
        DrawResources,
        Frame,
        draw_background,
        draw_explosion,
        draw_grid,
        draw_orbit,
        draw_planet,
        draw_ship,
    },
    meshes::{
        self,
        Meshes,
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
        let size   = window.size();
        let format = wgpu::TextureFormat::Bgra8UnormSrgb;

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
        let drawables = Drawables::new(&device, &meshes, format)
            .map_err(|err| Error::Drawables(err))?;

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format,
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
                    ui::Basic::new()
                )
            }
            UiOption::Conrod => {
                let screen = screen(&swap_chain_desc, scale_factor);

                Box::new(
                    ui::Conrod::new(&device, format, &screen)
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
                draw_grid(
                    &mut self.draw_res,
                    &mut frame,
                    game,
                );

                for orbit in game.state.active_orbits() {
                    draw_orbit(
                        &mut self.draw_res,
                        &mut frame,
                        &orbit,
                        game,
                    );
                }
                for planet in game.state.data.planets.values() {
                    draw_planet(
                        &mut self.draw_res,
                        &mut frame,
                        planet,
                        game,
                    );
                }
                for ship in game.state.data.ships.values() {
                    draw_ship(
                        &mut self.draw_res,
                        &mut frame,
                        ship,
                        game,
                    );
                }
                for explosion in game.state.data.explosions.values() {
                    draw_explosion(
                        &mut self.draw_res,
                        &mut frame,
                        explosion,
                        game,
                    );
                }

                self.ui
                    .draw(
                        &mut self.draw_res,
                        &mut frame,
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
    Drawables(drawables::Error),
    Font(rusttype::Error),
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
