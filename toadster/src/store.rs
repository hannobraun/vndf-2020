pub mod strong;
pub mod weak;


pub use self::{
    strong::Strong,
    weak::Weak,
};


use crate::handle;

pub trait Store<T> {
    fn get(&self, handle: &handle::Strong<T>) -> Option<&T>;
    fn get_mut(&mut self, handle: &handle::Strong<T>) -> Option<&mut T>;
}
