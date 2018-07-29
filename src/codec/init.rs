use bytes::{BufMut, BytesMut};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_codec::Encoder;

#[derive(Debug)]
pub struct InitPacket {}

pub struct Codec {}

impl Codec {
    pub fn new() -> Codec {
        Codec {}
    }
}

impl Encoder for Codec {
    type Item = InitPacket;
    type Error = io::Error;

    fn encode(&mut self, _packet: InitPacket, buf: &mut BytesMut) -> Result<(), io::Error> {
        // TODO: use initPacket instead of creating one...
        buf.reserve(128 + 32);
        // TODO random IV
        for i in 0..127 {
            buf.put_uint_be(i, 1);
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        buf.put_u32_be(timestamp.as_secs() as u32);
        Ok(())
    }
}
