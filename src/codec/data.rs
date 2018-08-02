use bytes::BytesMut;
use nom::*;
use std::io;
use std::str;
use tokio_codec::Decoder;

#[derive(Debug)]
pub struct DataPacket {
    version: u16,
    crc: u32,
    timestamp: u32,
    state: i16,
    hostname: String,
    service: String,
    plugin_output: String,
}

impl DataPacket {
    fn new(
        version: u16,
        crc: u32,
        timestamp: u32,
        state: i16,
        hostname: &[u8],
        service: &[u8],
        plugin_output: &[u8],
    ) -> DataPacket {
        DataPacket {
            version: version,
            crc: crc,
            timestamp: timestamp,
            state: state,
            hostname: str::from_utf8(hostname).unwrap().to_string(),
            service: str::from_utf8(service).unwrap().to_string(),
            plugin_output: str::from_utf8(plugin_output).unwrap().to_string(),
        }
    }
}

// taken from https://github.com/aerostitch/nscatools/blob/master/datapacket.go
named!(parse_data<&[u8], DataPacket>,do_parse!(
    // // parsing binary format
    version: be_u16 >>
    take!(2) >>
    crc: be_u32 >>
    timestamp: be_u32 >>
    state: be_i16 >>
    hostname: flat_map!(take!(64), take_until_and_consume!("\x00")) >> 
    service: flat_map!(take!(128), take_until_and_consume!("\x00")) >>
    plugin_output: flat_map!(take!(512), take_until_and_consume!("\x00")) >>

    // constructing DataPacket
    (DataPacket::new(version, crc, timestamp, state, hostname, service, plugin_output))
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
