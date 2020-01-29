use slotmap::{
    DefaultKey,
    SparseSecondaryMap,
    sparse_secondary,
};

use super::Handle;


pub struct SecondaryStore<T>(SparseSecondaryMap<DefaultKey, T>);

impl<T> SecondaryStore<T> {
    pub fn new() -> Self {
        Self(SparseSecondaryMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, handle: Handle, value: T) -> Option<T> {
        self.0.insert(handle.0, value)
    }

    pub fn remove(&mut self, handle: Handle) -> Option<T> {
        self.0.remove(handle.0)
    }

    pub fn get(&self, handle: Handle) -> Option<&T> {
        self.0.get(handle.0)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.0.iter())
    }

    pub fn values(&self) -> sparse_secondary::Values<DefaultKey, T> {
        self.0.values()
    }
}

impl<'a, T> IntoIterator for &'a SecondaryStore<T> {
    type Item     = (Handle, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub struct Iter<'a, T>(sparse_secondary::Iter<'a, DefaultKey, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (Handle(key), value))
    }
}