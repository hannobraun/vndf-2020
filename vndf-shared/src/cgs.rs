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


#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Handle(DefaultKey);
