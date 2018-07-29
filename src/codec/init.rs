use bytes::{BufMut, BytesMut};
use std::io;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_codec::Encoder;

#[derive(Debug)]
pub struct InitPacket {
    timestamp: Duration,
    iv: Vec<u8>,
}

impl InitPacket {
    pub fn new() -> InitPacket {
        let mut vec = Vec::new();
        for i in 0..127 {
            vec.push(i);
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        InitPacket {
            timestamp: timestamp,
            iv: vec,
        }
    }
}

pub struct Codec {}

impl Codec {
    pub fn new() -> Codec {
        Codec {}
    }
}

impl Encoder for Codec {
    type Item = InitPacket;
    type Error = io::Error;

    fn encode(&mut self, packet: InitPacket, buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(128 + 32);

        for i in packet.iv {
            buf.put_u32_be(i.into());
        }

        buf.put_u32_be(packet.timestamp.as_secs() as u32);
        Ok(())
    }
}
