use slotmap::{
    DefaultKey,
    DenseSlotMap,
};


pub type Handle   = DefaultKey;
pub type Store<T> = DenseSlotMap<DefaultKey, T>;
