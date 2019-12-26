use postcard;
use serde::{
    Deserialize,
    Serialize,
    de::DeserializeOwned,
};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum FromServer {
    Welcome,
}


pub fn serialize<T>(message: T, buf: &mut Vec<u8>) -> Result<(), Error>
    where T: Serialize
{
    let mut buf2 = [0; 1024];

    let serialized = postcard::to_slice(&message, &mut buf2)?;
    buf.extend(serialized.iter());

    Ok(())
}

pub fn deserialize<'de, T>(buf: &'de mut Vec<u8>) -> Result<Option<T>, Error>
    where T: DeserializeOwned
{
    let (message, bytes_taken) = match take_from_bytes(&buf) {
        Ok((message, bytes_taken))           => (Some(message), bytes_taken),
        Err(Error::DeserializeUnexpectedEnd) => (None, 0),
        Err(err)                             => return Err(err),
    };

    buf.drain(..bytes_taken);

    Ok(message)
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

    use super::{
        deserialize,
        serialize,
    };


    #[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
    pub struct Ping(u64);


    #[test]
    fn it_should_serialize_to_and_from_a_buffer() {
        let mut buf = Vec::new();

        let original_1 = Ping(1);
        let original_2 = Ping(2);

        serialize(original_1, &mut buf)
            .expect("Failed to serialize message");
        serialize(original_2, &mut buf)
            .expect("Failed to serialize message");

        let deserialized_1 = deserialize::<Ping>(&mut buf)
            .expect("Failed to deserialize message");
        let deserialized_2 = deserialize::<Ping>(&mut buf)
            .expect("Failed to deserialize message");

        assert_eq!(deserialized_1, Some(original_1));
        assert_eq!(deserialized_2, Some(original_2));
        assert_eq!(buf.len(), 0)
    }

    #[test]
    fn it_should_return_none_if_buffer_is_empty() {
        let mut buf = Vec::new();
        let deserialized = deserialize::<Ping>(&mut buf)
            .expect("Failed to deserialize message");
        assert_eq!(deserialized, None);
    }
}
