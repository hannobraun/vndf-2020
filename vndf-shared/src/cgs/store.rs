use std::{
    cell::Cell,
    marker::PhantomData,
};

use slotmap::{
    DefaultKey,
    DenseSlotMap,
    dense,
};

use crate::events;

use super::{
    Get,
    GetMut,
    Handle,
};


pub struct Store<T> {
    inner:     DenseSlotMap<DefaultKey, T>,
    to_remove: Cell<Vec<Handle<T>>>,
    removed:   events::Buf<Handle<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            inner:     DenseSlotMap::new(),
            to_remove: Cell::new(Vec::new()),
            removed:   events::Buf::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        Handle(self.inner.insert(value), PhantomData)
    }

    pub fn remove(&mut self, handle: Handle<T>) -> Option<T> {
        let result = self.inner.remove(handle.0);

        if result.is_some() {
            self.removed.sink().push(handle)
        }

        result
    }

    pub fn remove_later(&self, handle: Handle<T>) {
        let mut to_remove = self.to_remove.take();
        to_remove.push(handle);
        self.to_remove.set(to_remove);
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.inner.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
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

    pub fn apply_changes(&mut self) {
        let mut to_remove = self.to_remove.take();
        for handle in to_remove.drain(..) {
            self.remove(handle);
        }
        self.to_remove.set(to_remove);
    }

    pub fn removed(&mut self) -> events::Source<Handle<T>> {
        self.removed.source()
    }
}

impl<T> Get<T> for Store<T> {
    fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.get(handle)
    }
}

impl<T> GetMut<T> for Store<T> {
    fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
        self.get_mut(handle)
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item     = (Handle<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Store<T> {
    type Item     = (Handle<T>, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct Iter<'a, T>(dense::Iter<'a, DefaultKey, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Handle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (Handle(key, PhantomData), value))
    }
}


pub struct IterMut<'a, T>(dense::IterMut<'a, DefaultKey, T>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (Handle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (Handle(key, PhantomData), value))
    }
}


#[cfg(test)]
mod tests {
    use super::Store;


    #[test]
    fn it_should_remove_values_later() {
        let mut store = Store::new();

        store.insert(());
        store.insert(());

        for (handle, _) in &store {
            store.remove_later(handle);
        }

        assert_eq!(store.len(), 2);

        store.apply_changes();

        assert_eq!(store.len(), 0);
    }

    #[test]
    fn it_should_emit_remove_events() {
        let mut store = Store::new();

        let handle = store.insert(());

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed.len(), 0);

        store.remove(handle);

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed, vec![handle]);
    }
}
