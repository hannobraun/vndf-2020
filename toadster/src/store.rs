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

impl<T> Get<T> for &'_ T where T: Get<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        T::get(self, handle)
    }
}

impl<T> Get<T> for &'_ mut T where T: Get<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        T::get(self, handle)
    }
}


pub trait GetMut<T> {
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>) -> Option<&mut T>;
}

impl<T> GetMut<T> for &'_ mut T where T: GetMut<T> {
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>
    {
        T::get_mut(self, handle)
    }
}


pub trait Values<'r, T: 'r> {
    type Values: Iterator<Item=&'r T>;

    fn values(&'r self) -> Self::Values;
}

impl<'r, T: 'r> Values<'r, T> for &'_ T where T: Values<'r, T> {
    type Values = T::Values;

    fn values(&'r self) -> Self::Values {
        T::values(self)
    }
}

impl<'r, T: 'r> Values<'r, T> for &'_ mut T where T: Values<'r, T> {
    type Values = T::Values;

    fn values(&'r self) -> Self::Values {
        T::values(self)
    }
}


pub trait ValuesMut<'r, T: 'r> {
    type ValuesMut: Iterator<Item=&'r mut T>;

    fn values_mut(&'r mut self) -> Self::ValuesMut;
}

impl<'r, T: 'r> ValuesMut<'r, T> for &'_ mut T where T: ValuesMut<'r, T> {
    type ValuesMut = T::ValuesMut;

    fn values_mut(&'r mut self) -> Self::ValuesMut {
        T::values_mut(self)
    }
}
