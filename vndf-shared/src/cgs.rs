use slotmap::{
    DefaultKey,
    DenseSlotMap,
    SparseSecondaryMap,
    sparse_secondary,
};


pub type Handle            = DefaultKey;
pub type Store<T>          = DenseSlotMap<DefaultKey, T>;


pub struct SecondaryStore<T>(SparseSecondaryMap<DefaultKey, T>);

impl<T> SecondaryStore<T> {
    pub fn new() -> Self {
        Self(SparseSecondaryMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: Handle, value: T) -> Option<T> {
        self.0.insert(key, value)
    }

    pub fn remove(&mut self, key: Handle) -> Option<T> {
        self.0.remove(key)
    }

    pub fn get(&self, key: Handle) -> Option<&T> {
        self.0.get(key)
    }

    pub fn values(&self) -> impl Iterator<Item=&T> {
        self.0.values()
    }
}

impl<'a, T> IntoIterator for &'a SecondaryStore<T> {
    type Item     = (Handle, &'a T);
    type IntoIter = sparse_secondary::Iter<'a, DefaultKey, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
