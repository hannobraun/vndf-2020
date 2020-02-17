use std::sync::{
    Arc,
    Mutex,
};

use rinnsal::{
    EventBuf,
    EventSource,
};
use slotmap::{
    DefaultKey,
    DenseSlotMap,
    dense,
};

use crate::{
    Store,
    handle,
};


pub struct Strong<T> {
    inner:   DenseSlotMap<DefaultKey, T>,
    changes: Arc<Mutex<Changes<T>>>,
    removed: EventBuf<handle::Weak<T>>,
}

impl<T> Strong<T> {
    pub fn new() -> Self {
        Self {
            inner:   DenseSlotMap::new(),
            changes: Arc::new(Mutex::new(Changes::new())),
            removed: EventBuf::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, value: T) -> handle::Strong<T> {
        handle::Strong::new(
            self.inner.insert(value),
            self.changes.clone(),
        )
    }

    pub fn remove(&mut self, handle: impl Into<handle::Weak<T>>) -> Option<T> {
        let handle = handle.into();
        let result = self.inner.remove(handle.0);

        if result.is_some() {
            self.removed.sink().push(handle)
        }

        result
    }

    pub fn remove_later(&self, handle: handle::Strong<T>) {
        let mut changes = self.changes.lock().unwrap();
        changes.remove.push(handle);
    }

    pub fn get(&self, handle: impl Into<handle::Weak<T>>)
        -> Option<&T>
    {
        self.inner.get(handle.into().0)
    }

    pub fn get_mut(&mut self, handle: impl Into<handle::Weak<T>>)
        -> Option<&mut T>
    {
        self.inner.get_mut(handle.into().0)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner:   self.inner.iter(),
            changes: self.changes.clone(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner:   self.inner.iter_mut(),
            changes: self.changes.clone(),
        }
    }

    pub fn values(&self) -> dense::Values<DefaultKey, T> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> dense::ValuesMut<DefaultKey, T> {
        self.inner.values_mut()
    }

    pub fn apply_changes(&mut self) {
        let mut changes = self.changes.lock().unwrap();
        for handle in changes.remove.drain(..) {
            let result = self.inner.remove(handle.key);

            if result.is_some() {
                self.removed.sink().push((&handle).into())
            }
        }
    }

    pub fn removed(&mut self) -> EventSource<handle::Weak<T>> {
        self.removed.source()
    }
}

impl<T> Store<T> for Strong<T> {
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

impl<'a, T> IntoIterator for &'a Strong<T> {
    type Item     = (handle::Strong<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Strong<T> {
    type Item     = (handle::Strong<T>, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


pub struct Iter<'a, T> {
    inner:   dense::Iter<'a, DefaultKey, T>,
    changes: Arc<Mutex<Changes<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (handle::Strong<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|(key, value)|
                (handle::Strong::new(key, self.changes.clone()), value)
            )
    }
}


pub struct IterMut<'a, T> {
    inner:   dense::IterMut<'a, DefaultKey, T>,
    changes: Arc<Mutex<Changes<T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (handle::Strong<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|(key, value)|
                (handle::Strong::new(key, self.changes.clone()), value)
            )
    }
}


pub(crate) struct Changes<T> {
    remove: Vec<handle::Strong<T>>,
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
    use crate::{
        handle,
        store,
    };


    #[test]
    fn it_should_remove_values_later() {
        let mut store = store::Strong::new();

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
        let mut store = store::Strong::new();

        let strong_handle = store.insert(());
        let weak_handle: handle::Weak<()> = (&strong_handle).into();

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed.len(), 0);

        store.remove(strong_handle);

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed, vec![weak_handle]);
    }
}
