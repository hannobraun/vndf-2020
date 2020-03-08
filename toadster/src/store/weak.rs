use slotmap::{
    DefaultKey,
    SparseSecondaryMap,
    sparse_secondary,
};

use crate::{
    handle,
    store,
};


pub struct Weak<T>(SparseSecondaryMap<DefaultKey, T>);

impl<T> Weak<T> {
    pub fn new() -> Self {
        Self(SparseSecondaryMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, handle: impl Into<handle::Weak<T>>, value: T)
        -> Option<T>
    {
        self.0.insert(handle.into().key(), value)
    }

    pub fn remove(&mut self, handle: impl Into<handle::Weak<T>>) -> Option<T> {
        self.0.remove(handle.into().key())
    }

    pub fn get(&self, handle: impl Into<handle::Weak<T>>)
        -> Option<&T>
    {
        self.0.get(handle.into().key())
    }

    pub fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>
    {
        self.0.get_mut(handle.into().key())
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.0.iter())
    }

    pub fn values(&self) -> sparse_secondary::Values<DefaultKey, T> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> sparse_secondary::ValuesMut<DefaultKey, T> {
        self.0.values_mut()
    }
}

impl<T> store::Get<T> for Weak<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>)
        -> Option<&T>
    {
        self.get(handle)
    }

    fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>
    {
        self.get_mut(handle)
    }
}

impl<'r, T: 'r> store::Values<'r, T> for Weak<T> {
    type Values    = sparse_secondary::Values<'r, DefaultKey, T>;
    type ValuesMut = sparse_secondary::ValuesMut<'r, DefaultKey, T>;

    fn values(&'r self) -> Self::Values {
        self.values()
    }

    fn values_mut(&'r mut self) -> Self::ValuesMut {
        self.values_mut()
    }
}

impl<'a, T> IntoIterator for &'a Weak<T> {
    type Item     = (handle::Weak<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub struct Iter<'a, T>(sparse_secondary::Iter<'a, DefaultKey, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (handle::Weak<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (handle::Weak::new(key), value))
    }
}
