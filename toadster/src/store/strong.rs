use std::sync::{Arc, Mutex};

use log::debug;
use rinnsal::{EventBuf, EventSource};
use slotmap::{dense, DefaultKey, DenseSlotMap};

use crate::{handle, store};

#[derive(Debug)]
pub struct Strong<T> {
    inner: DenseSlotMap<DefaultKey, Entry<T>>,
    changes: Arc<Mutex<Changes>>,
    removed: EventBuf<handle::Weak<T>>,
}

impl<T> Strong<T> {
    pub fn new() -> Self {
        Self {
            inner: DenseSlotMap::new(),
            changes: Arc::new(Mutex::new(Changes::new())),
            removed: EventBuf::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, value: T) -> handle::Strong<T> {
        handle::Strong::from_key(
            self.inner.insert(Entry::new(value)),
            self.changes.clone(),
            false,
        )
    }

    pub fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        self.inner
            .get(handle.into().key())
            .map(|entry| &entry.value)
    }

    pub fn get_mut(
        &mut self,
        handle: impl Into<handle::Weak<T>>,
    ) -> Option<&mut T> {
        self.inner
            .get_mut(handle.into().key())
            .map(|entry| &mut entry.value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.inner.iter(),
            changes: self.changes.clone(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner: self.inner.iter_mut(),
            changes: self.changes.clone(),
        }
    }

    pub fn values(&self) -> Values<T> {
        Values(self.inner.values())
    }

    pub fn values_mut(&mut self) -> ValuesMut<T> {
        ValuesMut(self.inner.values_mut())
    }

    pub fn apply_changes(&mut self) {
        let mut changes = self.changes.lock().unwrap();

        for key in changes.track.drain(..) {
            let entry = self.inner.get_mut(key).unwrap();
            entry.track = true;
        }
        for key in changes.inc_count.drain(..) {
            let entry = self.inner.get_mut(key).unwrap();

            if entry.track {
                debug!("inc: {:?} ({})", key, entry.count);
            }

            entry.count += 1;
        }
        for key in changes.dec_count.drain(..) {
            let entry = self.inner.get_mut(key).unwrap();

            if entry.track {
                debug!("dec: {:?} ({})", key, entry.count);
            }

            entry.count -= 1;
            if entry.count == 0 {
                if entry.track {
                    debug!("del: {:?}", key);
                }

                self.inner.remove(key);
                self.removed.sink().push(handle::Weak::new(key));
            }
        }
    }

    pub fn removed(&mut self) -> EventSource<handle::Weak<T>> {
        self.removed.source()
    }
}

impl<T> store::Get<T> for Strong<T> {
    fn get(&self, handle: impl Into<handle::Weak<T>>) -> Option<&T> {
        self.get(handle)
    }
}

impl<T> store::GetMut<T> for Strong<T> {
    fn get_mut(
        &mut self,
        handle: impl Into<handle::Weak<T>>,
    ) -> Option<&mut T> {
        self.get_mut(handle)
    }
}

impl<'r, T: 'r> store::Values<'r, T> for Strong<T> {
    type Values = Values<'r, T>;

    fn values(&'r self) -> Self::Values {
        self.values()
    }
}

impl<'r, T: 'r> store::ValuesMut<'r, T> for Strong<T> {
    type ValuesMut = ValuesMut<'r, T>;

    fn values_mut(&'r mut self) -> Self::ValuesMut {
        self.values_mut()
    }
}

impl<'a, T> IntoIterator for &'a Strong<T> {
    type Item = (handle::Weak<T>, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Strong<T> {
    type Item = (handle::Weak<T>, &'a mut T);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Debug)]
struct Entry<T> {
    value: T,
    count: u64,
    track: bool,
}

impl<T> Entry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            count: 0,
            track: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Changes {
    pub(crate) inc_count: Vec<DefaultKey>,
    pub(crate) dec_count: Vec<DefaultKey>,
    pub(crate) track: Vec<DefaultKey>,
}

impl Changes {
    pub fn new() -> Self {
        Self {
            inc_count: Vec::new(),
            dec_count: Vec::new(),
            track: Vec::new(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: dense::Iter<'a, DefaultKey, Entry<T>>,
    changes: Arc<Mutex<Changes>>,
}

impl<'a, T> Iter<'a, T> {
    pub fn strong(self) -> IterStrong<'a, T> {
        IterStrong { inner: self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (handle::Weak<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(key, entry)| (handle::Weak::new(key), &entry.value))
    }
}

pub struct IterStrong<'a, T> {
    inner: Iter<'a, T>,
}

impl<'a, T> Iterator for IterStrong<'a, T> {
    type Item = (handle::Strong<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(handle, value)| {
            let handle = handle::Strong::from_handle(
                handle,
                self.inner.changes.clone(),
                false,
            );
            (handle, value)
        })
    }
}

pub struct IterMut<'a, T> {
    inner: dense::IterMut<'a, DefaultKey, Entry<T>>,
    changes: Arc<Mutex<Changes>>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn strong(self) -> IterMutStrong<'a, T> {
        IterMutStrong { inner: self }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (handle::Weak<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(key, entry)| (handle::Weak::new(key), &mut entry.value))
    }
}

pub struct IterMutStrong<'a, T> {
    inner: IterMut<'a, T>,
}

impl<'a, T> Iterator for IterMutStrong<'a, T> {
    type Item = (handle::Strong<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(handle, value)| {
            let handle = handle::Strong::from_handle(
                handle,
                self.inner.changes.clone(),
                false,
            );
            (handle, value)
        })
    }
}

pub struct Values<'a, T>(dense::Values<'a, DefaultKey, Entry<T>>);

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|entry| &entry.value)
    }
}

pub struct ValuesMut<'a, T>(dense::ValuesMut<'a, DefaultKey, Entry<T>>);

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|entry| &mut entry.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{handle, store};

    #[test]
    fn it_should_remove_component_when_all_handles_are_dropped() {
        let mut store = store::Strong::new();

        let handle = store.insert(());

        assert_eq!(store.len(), 1);

        drop(handle);
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

        drop(strong_handle);
        store.apply_changes();

        let removed: Vec<_> = store.removed().ready().collect();
        assert_eq!(removed, vec![weak_handle]);
    }
}
