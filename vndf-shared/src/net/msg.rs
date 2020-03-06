use std::fmt::Debug;

use postcard;
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};

use crate::{
    data,
    game::{
        Diagnostics,
        players::PlayerId,
    },
    input::Action,
};


pub trait Message : Send + Debug + DeserializeOwned + Serialize {
    fn write(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut buf2 = [0; 1024];

        let serialized = postcard::to_slice(self, &mut buf2)?;
        buf.extend(serialized.iter());

        Ok(())
    }

    fn read(buf: &mut Vec<u8>) -> Result<Option<Self>, Error> {
        let (message, num_bytes) = match take_from_bytes(&buf) {
            Ok((message, num_bytes))             => (Some(message), num_bytes),
            Err(Error::DeserializeUnexpectedEnd) => (None, 0),
            Err(err)                             => return Err(err),
        };

        buf.drain(..num_bytes);

        Ok(message)
    }
}

impl<T> Message for T where T: Send + Debug + DeserializeOwned + Serialize {}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum FromClient {
    Ping,
    Hello {
        color: [f32; 3],
    },
    Action(Action),
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum FromServer {
    Ping,
    Welcome(PlayerId),
    UpdateComponent(data::client::Component),
    RemoveComponent(data::client::Handle),
    InputHandled { seq: u64 },
    Diagnostics(Diagnostics),
}


fn take_from_bytes<'de, T>(buf: &'de [u8]) -> Result<(T, usize), Error>
    where T: Deserialize<'de>
{
    let (value, rest) = postcard::take_from_bytes(buf)?;
    Ok((value, buf.len() - rest.len()))
}


pub type Error = postcard::Error;


#[cfg(test)]
mod tests {
    use serde::{
        Deserialize,
        Serialize,
    };

    use super::Message as _;

    #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
    pub struct Ping(u64);


    #[test]
    fn it_should_serialize_to_and_from_a_buffer() {
        let mut buf = Vec::new();

        let original_1 = Ping(1);
        let original_2 = Ping(2);

        original_1.write(&mut buf)
            .expect("Failed to serialize message");
        original_2.write(&mut buf)
            .expect("Failed to serialize message");

        let deserialized_1 = Ping::read(&mut buf)
            .expect("Failed to deserialize message");
        let deserialized_2 = Ping::read(&mut buf)
            .expect("Failed to deserialize message");

        assert_eq!(deserialized_1, Some(original_1));
        assert_eq!(deserialized_2, Some(original_2));
        assert_eq!(buf.len(), 0)
    }

    #[test]
    fn it_should_return_none_if_buffer_is_empty() {
        let mut buf = Vec::new();
        let deserialized = Ping::read(&mut buf)
            .expect("Failed to deserialize message");
        assert_eq!(deserialized, None);
    }
}
