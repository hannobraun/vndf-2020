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
pub struct StrongHandle<T>(pub(crate) DefaultKey, PhantomData<T>);

impl<T> StrongHandle<T> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self(key, PhantomData)
    }
}

impl<T> Clone for StrongHandle<T> {
    fn clone(&self) -> Self {
        StrongHandle::new(self.0.clone())
    }
}

impl<T> Copy for StrongHandle<T> {}

impl<T> fmt::Debug for StrongHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StrongHandle<T>(")?;
        self.0.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for StrongHandle<T> {}

impl<T> PartialEq for StrongHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Hash for StrongHandle<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.hash(state)
    }
}
