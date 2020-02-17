use std::{
    any::TypeId,
    fmt,
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
};

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use slotmap::DefaultKey;

use crate::handle::{
    self,
    Untyped,
};


pub struct Weak<T> {
    inner: DefaultKey,
    kind:  Option<TypeId>,
    _data: PhantomData<T>,
}

impl<T> Weak<T> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self {
            inner: key,
            kind:  None,
            _data: PhantomData,
        }
    }

    pub(crate) fn key(&self) -> DefaultKey {
        self.inner
    }

    pub fn into_untyped(self) -> Weak<Untyped> where T: 'static {
        let mut handle = Weak::new(self.key());

        // Let's make sure there's something to distinguish handles of different
        // types. The whole point of untyped handles is being able to mix
        // handles of originally different types, but if they end up being equal
        // to each other, that's no good.
        handle.kind = Some(TypeId::of::<T>());

        handle
    }
}

impl<T> From<handle::Strong<T>> for Weak<T> {
    fn from(handle: handle::Strong<T>) -> Self {
        Self::new(handle.key())
    }
}

impl<T> From<&handle::Strong<T>> for Weak<T> {
    fn from(handle: &handle::Strong<T>) -> Self {
        Self::new(handle.key())
    }
}

impl<T> From<&handle::Weak<T>> for Weak<T> {
    fn from(handle: &handle::Weak<T>) -> Self {
        *handle
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Self::new(self.key().clone())
    }
}

impl<T> Copy for Weak<T> {}

impl<T> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "handle::Weak<T>(")?;
        self.key().fmt(f)?;
        write!(f, ", PhantomData)")?;

        Ok(())
    }
}

impl<T> Eq for Weak<T> {}

impl<T> PartialEq for Weak<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key().eq(&other.key()) && self.kind.eq(&other.kind)
    }
}

impl<T> Hash for Weak<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.key().hash(state);
        self.kind.hash(state);
    }
}

impl<T> Serialize for Weak<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.key().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Weak<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let key = DefaultKey::deserialize(deserializer)?;
        Ok(Self::new(key))
    }
}
