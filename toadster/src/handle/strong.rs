use std::{
    borrow::Borrow,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use backtrace::Backtrace;
use log::trace;
use slotmap::DefaultKey;

use crate::{
    handle::{Untyped, Weak},
    store::strong::Changes,
};

pub struct Strong<T> {
    inner: Weak<T>,
    changes: Arc<Mutex<Changes>>,
    track: bool,
    _data: PhantomData<T>,
}

impl<T> Strong<T> {
    pub(crate) fn from_key(
        key: DefaultKey,
        changes: Arc<Mutex<Changes>>,
        track: bool,
    ) -> Self {
        Self::from_handle(Weak::new(key), changes, track)
    }

    pub(crate) fn from_handle(
        inner: Weak<T>,
        changes: Arc<Mutex<Changes>>,
        track: bool,
    ) -> Self {
        if track {
            trace!("inc: {:?} {:?}", inner.key(), Backtrace::new());
        }

        {
            let mut changes = changes.lock().unwrap();
            changes.inc_count.push(inner.key());
        }

        Self {
            inner,
            changes,
            track,
            _data: PhantomData,
        }
    }

    pub(crate) fn key(&self) -> DefaultKey {
        self.inner.key()
    }

    pub fn track(&mut self) {
        self.track = true;
    }

    pub fn into_weak(&self) -> Weak<T> {
        self.inner.into()
    }

    pub fn into_untyped(self) -> Strong<Untyped>
    where
        T: 'static,
    {
        Strong::from_handle(
            self.inner.into_untyped(),
            self.changes.clone(),
            self.track,
        )
    }
}

impl<T> Clone for Strong<T> {
    fn clone(&self) -> Self {
        Self::from_handle(self.inner, self.changes.clone(), self.track)
    }
}

impl<T> Drop for Strong<T> {
    fn drop(&mut self) {
        if self.track {
            trace!("dec: {:?} {:?}", self.inner.key(), Backtrace::new());
        }

        let mut changes = self.changes.lock().unwrap();
        changes.dec_count.push(self.inner.key());
    }
}

impl<T> fmt::Debug for Strong<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Strong<T> {{ inner: ")?;
        self.inner.fmt(f)?;
        write!(f, ", changes: ")?;
        self.changes.fmt(f)?;
        write!(f, ", _data: PhantomData }}")?;

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
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.inner.hash(state)
    }
}

impl<T> Borrow<Weak<T>> for Strong<T> {
    fn borrow(&self) -> &Weak<T> {
        &self.inner
    }
}
