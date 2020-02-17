use std::{
    borrow::Borrow,
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
    handle::{
        Untyped,
        Weak,
    },
    store::strong::Changes,
};


pub struct Strong<T> {
    inner:   Weak<T>,
    changes: Arc<Mutex<Changes<T>>>,
    _data:   PhantomData<T>,
}

impl<T> Strong<T> {
    pub(crate) fn new(key: DefaultKey, changes: Arc<Mutex<Changes<T>>>)
        -> Self
    {
        Self {
            inner: Weak::new(key),
            changes,
            _data: PhantomData,
        }
    }

    pub(crate) fn key(&self) -> &DefaultKey {
        &self.inner.0
    }

    pub fn into_untyped(self) -> Strong<Untyped> {
        Strong {
            inner: Weak::new(self.inner.0),
            // This is a short-term hack. It makes no difference right now, and
            // the type parameter will be gone from `Changes` shortly.
            changes: unsafe { std::mem::transmute(self.changes) },
            _data:   PhantomData,
        }
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::new(self.inner.0, self.changes.clone())
    }
}

impl<T> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Strong<T>(")?;
        self.inner.0.fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Strong<T> {}

impl<T> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.0.eq(&other.inner.0)
    }
}

// Most only rely on the inner `Weak`, to make sure the `Hash` and `Borrow`
// implementations interact well with collections.
impl<T> Hash for Strong<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.inner.hash(state)
    }
}

impl<T> Borrow<Weak<T>> for Strong<T> {
    fn borrow(&self) -> &Weak<T> {
        &self.inner
    }
}
