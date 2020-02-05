pub mod secondary_store;
pub mod store;


pub use self::{
    secondary_store::SecondaryStore,
    store::Store,
};


use serde::{
    Deserialize,
    Serialize,
};
use slotmap::DefaultKey;


#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Handle(DefaultKey);


pub trait GetMut<T> {
    fn get_mut(&mut self, handle: Handle) -> Option<&mut T>;
}
