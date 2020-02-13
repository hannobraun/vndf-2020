pub mod handle;
pub mod secondary_store;
pub mod store;


pub use self::{
    handle::Handle,
    secondary_store::SecondaryStore,
    store::Store,
};


pub trait Get<T> {
    fn get(&self, handle: &Handle<T>) -> Option<&T>;
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T>;
}
