use std::cell::Cell;

use slotmap::{
    DefaultKey,
    DenseSlotMap,
    dense,
};
use vndf_events as events;

use super::{
    Get,
    GetMut,
    StrongHandle,
};


pub struct StrongStore<T> {
    inner:     DenseSlotMap<DefaultKey, T>,
    changes:   Cell<Changes<T>>,
    removed:   events::Buf<StrongHandle<T>>,
}

impl<T> StrongStore<T> {
    pub fn new() -> Self {
        Self {
            inner:   DenseSlotMap::new(),
            changes: Cell::new(Changes::new()),
            removed: events::Buf::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, value: T) -> StrongHandle<T> {
        StrongHandle::new(self.inner.insert(value))
    }

    pub fn remove(&mut self, handle: StrongHandle<T>) -> Option<T> {
        let result = self.inner.remove(handle.0);

        if result.is_some() {
            self.removed.sink().push(handle)
        }

        result
    }

    pub fn remove_later(&self, handle: StrongHandle<T>) {
        let mut changes = self.changes.take();
        changes.remove.push(handle);
        self.changes.set(changes);
    }

    pub fn get(&self, handle: &StrongHandle<T>) -> Option<&T> {
        self.inner.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: &StrongHandle<T>) -> Option<&mut T> {
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
        let mut changes = self.changes.take();
        for handle in changes.remove.drain(..) {
            self.remove(handle);
        }
        self.changes.set(changes);
    }

    pub fn removed(&mut self) -> events::Source<StrongHandle<T>> {
        self.removed.source()
    }
}

impl<T> Get<T> for StrongStore<T> {
    fn get(&self, handle: &StrongHandle<T>) -> Option<&T> {
        self.get(handle)
    }
}

impl<T> GetMut<T> for StrongStore<T> {
    fn get_mut(&mut self, handle: &StrongHandle<T>) -> Option<&mut T> {
        self.get_mut(handle)
    }
}

impl<'a, T> IntoIterator for &'a StrongStore<T> {
    type Item     = (StrongHandle<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut StrongStore<T> {
    type Item     = (StrongHandle<T>, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct Iter<'a, T>(dense::Iter<'a, DefaultKey, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (StrongHandle<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (StrongHandle::new(key), value))
    }
}


pub struct IterMut<'a, T>(dense::IterMut<'a, DefaultKey, T>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (StrongHandle<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
            .map(|(key, value)| (StrongHandle::new(key), value))
    }
}


struct Changes<T> {
    remove: Vec<StrongHandle<T>>,
}

impl<T> Changes<T> {
    pub fn new() -> Self {
        Self {
            remove: Vec::new(),
        }
    }
}

impl<T> Default for Changes<T> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::StrongStore;


    #[test]
    fn it_should_remove_values_later() {
        let mut store = StrongStore::new();

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
        let mut store = StrongStore::new();

        let handle = store.insert(());

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed.len(), 0);

        store.remove(handle);

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed, vec![handle]);
    }
}
