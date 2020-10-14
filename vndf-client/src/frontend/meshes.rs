use lyon::{
    path,
    tessellation::{
        BuffersBuilder, FillAttributes, FillOptions, FillTessellator, TessellationError,
        VertexBuffers,
    },
};

use crate::graphics::vertices;

pub type Vertex = [f32; 2];
pub type Index = u16;

pub struct Meshes {
    pub ship: Mesh,
    pub square: Mesh,
}

impl Meshes {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            ship: Mesh::new(vertices::SHIP)?,
            square: Mesh::new(vertices::SQUARE)?,
        })
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
}

impl Mesh {
    pub fn new(vertices: &[vertices::Vertex]) -> Result<Self, Error> {
        let mut builder = path::Builder::new();
        builder.polygon(vertices);

        let path = builder.build();

        let mut buffers = VertexBuffers::<Vertex, Index>::new();

        let mut tesselator = FillTessellator::new();
        tesselator.tessellate(
            &path,
            &FillOptions::default(),
            &mut BuffersBuilder::new(
                &mut buffers,
                |point: vertices::Vertex, _: FillAttributes| point.to_array(),
            ),
        )?;

        Ok(Self {
            vertices: buffers.vertices,
            indices: buffers.indices,
        })
    }
}

pub type Error = TessellationError;
