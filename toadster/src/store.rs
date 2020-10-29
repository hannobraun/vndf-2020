pub mod strong;
pub mod weak;

pub use self::{strong::Strong, weak::Weak};

use crate::handle;

pub trait Get<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T>;
}

impl<G, T> Get<T> for &'_ G
where
    G: Get<T>,
{
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        G::get(self, handle)
    }
}

impl<G, T> Get<T> for &'_ mut G
where
    G: Get<T>,
{
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        G::get(self, handle)
    }
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>;
}

impl<G, T> GetMut<T> for &'_ mut G
where
    G: GetMut<T>,
{
    fn get_mut(
        &mut self,
        handle: impl Into<handle::Weak<T>>,
    ) -> Option<&mut T> {
        G::get_mut(self, handle)
    }
}

pub trait Values<'r, T: 'r> {
    type Values: Iterator<Item = &'r T>;

    fn values(&'r self) -> Self::Values;
}

impl<'r, V, T: 'r> Values<'r, T> for &'_ V
where
    V: Values<'r, T>,
{
    type Values = V::Values;

    fn values(&'r self) -> Self::Values {
        V::values(self)
    }
}

impl<'r, V, T: 'r> Values<'r, T> for &'_ mut V
where
    V: Values<'r, T>,
{
    type Values = V::Values;

    fn values(&'r self) -> Self::Values {
        V::values(self)
    }
}

pub trait ValuesMut<'r, T: 'r> {
    type ValuesMut: Iterator<Item = &'r mut T>;

    fn values_mut(&'r mut self) -> Self::ValuesMut;
}

impl<'r, V, T: 'r> ValuesMut<'r, T> for &'_ mut V
where
    V: ValuesMut<'r, T>,
{
    type ValuesMut = V::ValuesMut;

    fn values_mut(&'r mut self) -> Self::ValuesMut {
        V::values_mut(self)
    }
}
