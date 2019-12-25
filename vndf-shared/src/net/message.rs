use postcard;
use serde::{
    Deserialize,
    Serialize,
};


#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Message {
    Ping(u64),
}

impl Message {
    pub fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), Error> {
        let mut buf2 = [0; 1024];

        let serialized = postcard::to_slice(self, &mut buf2)?;
        buf.extend(serialized.iter());

        Ok(())
    }

    pub fn deserialize(buf: &mut Vec<u8>) -> Result<Option<Self>, Error> {
        let (message, rest) = match postcard::take_from_bytes(&buf) {
            Ok((message, rest))                  => (Some(message), rest),
            Err(Error::DeserializeUnexpectedEnd) => (None, buf.as_ref()),
            Err(err)                             => return Err(err),
        };

        let bytes_taken = buf.len() - rest.len();
        buf.drain(..bytes_taken);

        Ok(message)
    }
}


pub type Error = postcard::Error;


#[cfg(test)]
mod tests {
    use super::Message;


    #[test]
    fn it_should_serialize_to_and_from_a_buffer() {
        let mut buf = Vec::new();

        let original_1 = Message::Ping(1);
        let original_2 = Message::Ping(2);

        original_1.serialize(&mut buf)
            .expect("Failed to serialize message");
        original_2.serialize(&mut buf)
            .expect("Failed to serialize message");

        let deserialized_1 = Message::deserialize(&mut buf)
            .expect("Failed to deserialize message");
        let deserialized_2 = Message::deserialize(&mut buf)
            .expect("Failed to deserialize message");

        assert_eq!(deserialized_1, Some(original_1));
        assert_eq!(deserialized_2, Some(original_2));
        assert_eq!(buf.len(), 0)
    }

    #[test]
    fn it_should_return_none_if_buffer_is_empty() {
        let mut buf = Vec::new();
        let deserialized = Message::deserialize(&mut buf)
            .expect("Failed to deserialize message");
        assert_eq!(deserialized, None);
    }
}
