pub mod strong;
pub mod weak;


use crate::StrongHandle;

pub trait Store<T> {
    fn get(&self, handle: &StrongHandle<T>) -> Option<&T>;
    fn get_mut(&mut self, handle: &StrongHandle<T>) -> Option<&mut T>;
}
