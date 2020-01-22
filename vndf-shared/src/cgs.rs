use slotmap::{
    DefaultKey,
    DenseSlotMap,
    SparseSecondaryMap,
};


pub type Handle            = DefaultKey;
pub type Store<T>          = DenseSlotMap<DefaultKey, T>;
pub type SecondaryStore<T> = SparseSecondaryMap<DefaultKey, T>;
