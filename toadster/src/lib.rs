pub mod handle;
pub mod store;


pub use self::{
    handle::StrongHandle,
    store::{
        strong::StrongStore,
        weak::WeakStore,
    },
};


pub trait Get<T> {
    fn get(&self, handle: &StrongHandle<T>) -> Option<&T>;
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: &StrongHandle<T>) -> Option<&mut T>;
}
