pub mod strong;
pub mod weak;


pub use self::{
    strong::Strong,
    weak::Weak,
};

use crate::handle;


pub trait Get<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>)
        -> Option<&T>;
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>;
}
