use postcard::{
    self,
    Error,
};
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

    pub fn deserialize(buf: &mut Vec<u8>) -> Result<Self, Error> {
        let (message, rest) = postcard::take_from_bytes(&buf)?;

        let bytes_taken = buf.len() - rest.len();
        buf.drain(..bytes_taken);

        Ok(message)
    }
}


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

        assert_eq!(original_1, deserialized_1);
        assert_eq!(original_2, deserialized_2);
        assert_eq!(buf.len(), 0)
    }
}
