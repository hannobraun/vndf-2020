pub mod secondary_store;
pub mod store;


pub use self::{
    secondary_store::SecondaryStore,
    store::Store,
};


use std::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
};

use serde::{
    Deserialize,
    Serialize,
};
use slotmap::DefaultKey;


#[derive(Deserialize, Serialize)]
pub struct Handle<T>(DefaultKey, PhantomData<T>);

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle(self.0.clone(), PhantomData)
    }
}

impl<T> Copy for Handle<T> {}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle<T>(")?;
        self.0.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.hash(state)
    }
}


pub trait Get<T> {
    fn get(&self, handle: &Handle<T>) -> Option<&T>;
}

pub trait GetMut<T> {
    fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T>;
}
