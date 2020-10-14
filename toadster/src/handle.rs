pub mod generic;
pub mod strong;
pub mod weak;

pub use self::{generic::Handle, strong::Strong, weak::Weak};

pub enum Untyped {}
