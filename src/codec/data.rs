use bytes::BytesMut;
use nom::*;
use std::io;
use tokio_codec::Decoder;

#[derive(Debug)]
pub struct DataPacket {
    version: i16,
    crc: u32,
    timestamp: u32,
    state: i16,
    // hostname: String,
    // service: String,
    // plugin_output: String,
}

// taken from https://github.com/aerostitch/nscatools/blob/master/datapacket.go
named!(parse_data<&[u8], DataPacket>,do_parse!(

    // // parsing binary format
    version: be_i16 >>
    crc: be_u32 >>
    timestamp: be_u32 >>
    state: be_i16 >>
    hostname: take_until_and_consume!("\x00") >>
    service: take_until_and_consume!("\x00") >>
    plugin_output: take_until_and_consume!("\x00") >>

    // constructing DataPacket
    (DataPacket {version, crc, timestamp, state})
  )
);

#[derive(Clone, Default, PartialEq)]
pub struct Codec {}

impl Codec {
    pub fn new() -> Codec {
        Codec {}
    }
}

impl Decoder for Codec {
    type Item = DataPacket;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        let (consumed, f) = match parse_data(buf) {
            Err(e) => {
                if e.is_incomplete() {
                    return Ok(None);
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("parse error: {:?}", e),
                    ));
                }
            }
            Ok((i, frame)) => (buf.offset(i), frame),
        };

        buf.split_to(consumed);

        Ok(Some(f))
    }
}
