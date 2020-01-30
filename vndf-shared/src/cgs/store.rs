use slotmap::{
    DefaultKey,
    DenseSlotMap,
    dense,
};

use super::Handle;


pub struct Store<T> {
    inner: DenseSlotMap<DefaultKey, T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            inner: DenseSlotMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, value: T) -> Handle {
        Handle(self.inner.insert(value))
    }

    pub fn remove(&mut self, handle: Handle) -> Option<T> {
        self.inner.remove(handle.0)
    }

    pub fn get(&self, handle: Handle) -> Option<&T> {
        self.inner.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        self.inner.get_mut(handle.0)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.inner.iter_mut())
    }

    pub fn values(&self) -> dense::Values<DefaultKey, T> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> dense::ValuesMut<DefaultKey, T> {
        self.inner.values_mut()
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item     = (Handle, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Store<T> {
    type Item     = (Handle, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct Iter<'a, T>(dense::Iter<'a, DefaultKey, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (Handle(key), value))
    }
}


pub struct IterMut<'a, T>(dense::IterMut<'a, DefaultKey, T>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (Handle(key), value))
    }
}
