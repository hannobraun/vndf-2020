use lyon::{
    path,
    tessellation::{
        BuffersBuilder,
        FillAttributes,
        FillOptions,
        FillTessellator,
        TessellationError,
        VertexBuffers,
    },
};

use crate::graphics::vertices::{
    self,
    Vertex,
};


pub struct Meshes {
    pub ship: Mesh,
}

impl Meshes {
    pub fn new() -> Result<Self, Error> {
        Ok(
            Self {
                ship: Mesh::new(vertices::SHIP)?,
            }
        )
    }
}


pub struct Mesh {
    pub vertices: Vec<[f32; 2]>,
    pub indices:  Vec<u16>,
}

impl Mesh {
    pub fn new(vertices: &[Vertex]) -> Result<Self, Error> {
        let mut builder = path::Builder::new();
        builder.polygon(vertices);

        let path = builder.build();

        let mut buffers = VertexBuffers::<[f32; 2], u16>::new();

        let mut tesselator = FillTessellator::new();
        tesselator.tessellate(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(
                &mut buffers,
                |point: Vertex, _: FillAttributes| point.to_array(),
            )
        )?;

        Ok(
            Self {
                vertices: buffers.vertices,
                indices:  buffers.indices,
            }
        )
    }
}


pub type Error = TessellationError;
