use slotmap::{
    DefaultKey,
    DenseSlotMap,
    dense,
};

use super::Handle;


pub struct Store<T>(DenseSlotMap<DefaultKey, T>);

impl<T> Store<T> {
    pub fn new() -> Self {
        Self(DenseSlotMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, value: T) -> Handle {
        self.0.insert(value)
    }

    pub fn remove(&mut self, key: Handle) -> Option<T> {
        self.0.remove(key)
    }

    pub fn get(&self, key: Handle) -> Option<&T> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: Handle) -> Option<&mut T> {
        self.0.get_mut(key)
    }

    pub fn iter(&self) -> dense::Iter<DefaultKey, T> {
        self.0.iter()
    }

    pub fn values(&self) -> dense::Values<DefaultKey, T> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> dense::ValuesMut<DefaultKey, T> {
        self.0.values_mut()
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item     = (Handle, &'a T);
    type IntoIter = dense::Iter<'a, DefaultKey, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Store<T> {
    type Item     = (Handle, &'a mut T);
    type IntoIter = dense::IterMut<'a, DefaultKey, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
