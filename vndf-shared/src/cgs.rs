pub mod secondary_store;
pub mod store;


pub use self::{
    secondary_store::SecondaryStore,
    store::Store,
};


use slotmap::DefaultKey;


pub type Handle            = DefaultKey;
