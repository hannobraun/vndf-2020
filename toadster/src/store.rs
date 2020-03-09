pub mod strong;
pub mod weak;


pub use self::{
    strong::Strong,
    weak::Weak,
};

use crate::handle;


pub trait Get<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T>;
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>) -> Option<&mut T>;
}


pub trait Values<'r, T: 'r> {
    type Values:    Iterator<Item=&'r T>;
    type ValuesMut: Iterator<Item=&'r mut T>;

    fn values(&'r self) -> Self::Values;
    fn values_mut(&'r mut self) -> Self::ValuesMut;
}
