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

use crate::handle::{
    self,
    Untyped,
};


#[derive(Deserialize, Serialize)]
pub struct Weak<T>(pub(crate) DefaultKey, PhantomData<T>);

impl<T> Weak<T> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self(key, PhantomData)
    }

    pub fn into_untyped(self) -> Weak<Untyped> {
        Weak::new(self.0)
    }
}

impl<T> From<handle::Strong<T>> for Weak<T> {
    fn from(handle: handle::Strong<T>) -> Self {
        Self::new(*handle.key())
    }
}

impl<T> From<&handle::Strong<T>> for Weak<T> {
    fn from(handle: &handle::Strong<T>) -> Self {
        Self::new(*handle.key())
    }
}

impl<T> From<&handle::Weak<T>> for Weak<T> {
    fn from(handle: &handle::Weak<T>) -> Self {
        *handle
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T> Copy for Weak<T> {}

impl<T> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Weak<T>(")?;
        self.0.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Weak<T> {}

impl<T> PartialEq for Weak<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Hash for Weak<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.hash(state)
    }
}
