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
pub struct Strong<T>(pub(crate) DefaultKey, PhantomData<T>);

impl<T> Strong<T> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self(key, PhantomData)
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Strong<T>(")?;
        self.0.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Strong<T> {}

impl<T> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Hash for Strong<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.hash(state)
    }
}
