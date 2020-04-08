pub type Vertex = [f32; 2];


pub const POINTER: &[Vertex] = &[
    [ 0.5,  0.5],
    [ 0.0, -0.5],
    [-0.5,  0.5],
];

pub const SHIP: &[Vertex] = &[
    [ 0.6,  0.0],
    [-0.4,  0.4],
    [-0.1,  0.0],
    [-0.4, -0.4],
];

pub const SQUARE: &[Vertex] = &[
    [ 0.5,  0.5],
    [ 0.5, -0.5],
    [-0.5, -0.5],
    [-0.5,  0.5],
];
