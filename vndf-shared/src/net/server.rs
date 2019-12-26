use serde::{
    Deserialize,
    Serialize,
};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Welcome,
}
