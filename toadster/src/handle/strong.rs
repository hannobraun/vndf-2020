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
    pub(crate) fn from_key(key: DefaultKey, changes: Arc<Mutex<Changes<T>>>)
        -> Self
    {
        Self::from_handle(Weak::new(key), changes)
    }

    pub(crate) fn from_handle(inner: Weak<T>, changes: Arc<Mutex<Changes<T>>>)
        -> Self
    {
        Self {
            inner,
            changes,
            _data: PhantomData,
        }
    }

    pub(crate) fn key(&self) -> DefaultKey {
        self.inner.key()
    }

    pub fn into_weak(&self) -> Weak<T> {
        self.inner.into()
    }

    pub fn into_untyped(self) -> Strong<Untyped> where T: 'static {
        // This is a short-term hack. It makes no difference right now, and the
        // type parameter will be gone from `Changes` shortly.
        let changes = unsafe { std::mem::transmute(self.changes) };
        Strong::from_handle(self.inner.into_untyped(), changes)
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::from_handle(self.inner, self.changes.clone())
    }
}

impl<T> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Strong<T>(")?;
        self.inner.key().fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Strong<T> {}

impl<T> PartialEq for Strong<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.key().eq(&other.inner.key())
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
