use bytes::{BufMut, BytesMut};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Packet<T: ToBytes> {
    pub packet_type: PacketType,
    pub payload: T,
}

pub trait ToBytes {
    fn to_bytes(&self, buf: &mut BytesMut) -> Result<(), io::Error>;
}

#[derive(Debug)]
pub enum PacketType {
    INIT,
    DATA,
}

#[derive(Debug)]
pub struct InitPacket {}

impl ToBytes for InitPacket {
    fn to_bytes(&self, buf: &mut BytesMut) -> Result<(), io::Error> {
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

#[derive(Debug)]
pub struct DataPacket {}

impl ToBytes for DataPacket {
    fn to_bytes(&self, buf: &mut BytesMut) -> Result<(), io::Error> {
        // TODO
        Ok(())
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct NSCACodec {
    pos: usize,
}

impl NSCACodec {
    pub fn new() -> NSCACodec {
        NSCACodec { pos: 0 }
    }
}

impl Decoder for NSCACodec {
    type Item = Packet<DataPacket>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO: nom?
        Ok(Some(Packet {
            packet_type: PacketType::DATA,
            payload: DataPacket {},
        }))
    }
}

impl Encoder for NSCACodec {
    type Item = Packet<InitPacket>;
    type Error = io::Error;

    fn encode(&mut self, packet: Packet<InitPacket>, buf: &mut BytesMut) -> Result<(), io::Error> {
        packet.payload.to_bytes(buf)
    }
}
