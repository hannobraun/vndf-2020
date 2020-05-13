use std::io::{
    self,
    Cursor,
};

use zerocopy::AsBytes;


pub trait Shader {
    type Kind;
    type Uniforms: AsBytes + Default;

    fn code() -> &'static [u8];

    fn load(device: &wgpu::Device)
        -> Result<wgpu::ShaderModule, io::Error>
    {
        let code = Self::code();

        let module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(code))?,
        );

        Ok(module)
    }
}


pub struct Vert;
pub struct Frag;


macro_rules! shader {
    (
        $kind:ty,
        $name_s:ident,
        $name_m:ident,
        $path:expr,
        Uniforms {
            $(
                $u_name:ident: $u_ty:ident,
            )*
        },
    ) => {
        pub struct $name_s;

        impl crate::frontends::shaders::Shader for $name_s {
            type Kind     = $kind;
            type Uniforms = $name_m::Uniforms;

            fn code() -> &'static [u8] {
                &include_bytes!($path)[..]
            }
        }

        pub mod $name_m {
            #[derive(zerocopy::AsBytes, Default)]
            #[repr(packed)]
            pub struct Uniforms {
                $(
                    pub $u_name: crate::frontends::uniforms::$u_ty,
                )*
            }
        }
    };
}

macro_rules! vertex_shader {
    (
        $name_s:ident,
        $name_m:ident,
        $path:expr,
        Uniforms {
            $(
                $u_name:ident: $u_ty:ident,
            )*
        },
    ) => {
        shader!(
            crate::frontends::shaders::Vert,
            $name_s,
            $name_m,
            $path,
            Uniforms {
                $(
                    $u_name: $u_ty,
                )*
            },
        );
    };
}

macro_rules! fragment_shader {
    (
        $name_s:ident,
        $name_m:ident,
        $path:expr,
        Uniforms {
            $(
                $u_name:ident: $u_ty:ident,
            )*
        },
    ) => {
        shader!(
            crate::frontends::shaders::Frag,
            $name_s,
            $name_m,
            $path,
            Uniforms {
                $(
                    $u_name: $u_ty,
                )*
            },
        );
    };
}


pub mod vert {
    vertex_shader!(
        Simple,
        simple,
        "shaders/spv/simple.vert.spv",
        Uniforms {
            transform: Mat4,
        },
    );
}

pub mod frag {
    fragment_shader!(
        Explosion,
        explosion,
        "shaders/spv/explosion.frag.spv",
        Uniforms {
            strength_total: Float,
            strength_left:  Float,
        },
    );
    fragment_shader!(
        Orbit,
        orbit,
        "shaders/spv/orbit.frag.spv",
        Uniforms {
            color:         Color,
            u_per_pixel:   Vec2,
            orbiter_angle: Float,
            orbiter_dir:   Float,
        },
    );
    fragment_shader!(
        Planet,
        planet,
        "shaders/spv/planet.frag.spv",
        Uniforms {
            color: Color,
        },
    );
    fragment_shader!(
        Simple,
        simple,
        "shaders/spv/simple.frag.spv",
        Uniforms {
            color: Color,
        },
    );
}
