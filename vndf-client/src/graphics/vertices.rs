use euclid::{
    Point2D,
    UnknownUnit,
};


pub type Vertex = Point2D<f32, UnknownUnit>;


pub const SHIP: &[Vertex] = &[
    Vertex::new( 0.6,  0.0),
    Vertex::new(-0.4,  0.4),
    Vertex::new(-0.1,  0.0),
    Vertex::new(-0.4, -0.4),
];

pub const SQUARE: &[Vertex] = &[
    Vertex::new(-0.5, -0.5),
    Vertex::new( 0.5, -0.5),
    Vertex::new( 0.5,  0.5),
    Vertex::new(-0.5,  0.5),
];
