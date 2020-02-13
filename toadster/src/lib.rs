pub mod handle;
pub mod secondary_store;
pub mod store;


pub use self::{
    handle::StrongHandle,
    secondary_store::SecondaryStore,
    store::StrongStore,
};


pub trait Get<T> {
    fn get(&self, handle: &StrongHandle<T>) -> Option<&T>;
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: &StrongHandle<T>) -> Option<&mut T>;
}
