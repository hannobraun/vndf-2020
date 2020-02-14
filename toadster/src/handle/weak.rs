use std::marker::PhantomData;

use serde::{
    Deserialize,
    Serialize,
};
use slotmap::DefaultKey;

use crate::handle;


#[derive(Deserialize, Serialize)]
pub struct Weak<T>(pub(crate) DefaultKey, PhantomData<T>);

impl<T> From<&handle::Strong<T>> for Weak<T> {
    fn from(handle: &handle::Strong<T>) -> Self {
        Self(handle.key, PhantomData)
    }
}

impl<T> Eq for Weak<T> {}

impl<T> PartialEq for Weak<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
