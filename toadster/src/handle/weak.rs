use std::marker::PhantomData;

use serde::{
    Deserialize,
    Serialize,
};
use slotmap::DefaultKey;


#[derive(Deserialize, Serialize)]
pub struct Weak<T>(DefaultKey, PhantomData<T>);
