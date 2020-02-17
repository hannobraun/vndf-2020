use std::{
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
    sync::{
        Arc,
        Mutex,
    },
};

use slotmap::DefaultKey;

use crate::{
    handle::Untyped,
    store::strong::Changes,
};


pub struct Strong<T> {
    pub(crate) key: DefaultKey,

    changes: Arc<Mutex<Changes<T>>>,
    _data:   PhantomData<T>,
}

impl<T> Strong<T> {
    pub(crate) fn new(key: DefaultKey, changes: Arc<Mutex<Changes<T>>>)
        -> Self
    {
        Self {
            key,
            changes,
            _data: PhantomData,
        }
    }

    pub fn into_untyped(self) -> Strong<Untyped> {
        Strong {
            key:     self.key,
            // This is a short-term hack. It makes no difference right now, and
            // the type parameter will be gone from `Changes` shortly.
            changes: unsafe { std::mem::transmute(self.changes) },
            _data:   PhantomData,
        }
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::new(self.key.clone(), self.changes.clone())
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
