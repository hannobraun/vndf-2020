use std::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
};

use slotmap::DefaultKey;


pub struct Strong<T> {
    pub(crate) key: DefaultKey,

    _data: PhantomData<T>,
}

impl<T> Strong<T> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self {
            key,
            _data: PhantomData,
        }
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::new(self.key.clone())
    }
}

impl<T> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Strong<T>(")?;
        self.key.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Strong<T> {}

impl<T> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<T> Hash for Strong<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.key.hash(state)
    }
}
