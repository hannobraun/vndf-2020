use postcard;
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};


pub trait Message : DeserializeOwned + Serialize {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut buf2 = [0; 1024];

        let serialized = postcard::to_slice(self, &mut buf2)?;
        buf.extend(serialized.iter());

        Ok(())
    }

    fn deserialize(buf: &mut Vec<u8>) -> Result<Option<Self>, Error> {
        let (message, bytes_taken) = match take_from_bytes(&buf) {
            Ok((message, bytes_taken))           => (Some(message), bytes_taken),
            Err(Error::DeserializeUnexpectedEnd) => (None, 0),
            Err(err)                             => return Err(err),
        };

        buf.drain(..bytes_taken);

        Ok(message)
    }
}

impl<T> Message for T where T: DeserializeOwned + Serialize {}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FromServer {
    Welcome,
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
    use super::Message as _;


    mod ping {
        use serde::{
            Deserialize,
            Serialize,
        };

        #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
        pub struct Ping(pub u64);
    }
    use self::ping::Ping;


    #[test]
    fn it_should_serialize_to_and_from_a_buffer() {
        let mut buf = Vec::new();

        let original_1 = Ping(1);
        let original_2 = Ping(2);

        original_1.serialize(&mut buf)
            .expect("Failed to serialize message");
        original_2.serialize(&mut buf)
            .expect("Failed to serialize message");

        let deserialized_1 = Ping::deserialize(&mut buf)
            .expect("Failed to deserialize message");
        let deserialized_2 = Ping::deserialize(&mut buf)
            .expect("Failed to deserialize message");

        assert_eq!(deserialized_1, Some(original_1));
        assert_eq!(deserialized_2, Some(original_2));
        assert_eq!(buf.len(), 0)
    }

    #[test]
    fn it_should_return_none_if_buffer_is_empty() {
        let mut buf = Vec::new();
        let deserialized = Ping::deserialize(&mut buf)
            .expect("Failed to deserialize message");
        assert_eq!(deserialized, None);
    }
}
