use byteorder::{BigEndian, ByteOrder};
use std::io;
use std::io::Error;
use tokio;
use tokio::io::copy;
use tokio::prelude::*;

pub(crate) struct InitPacket {
    iv: Vec<u8>,
    timestamp: u32,
}

fn new_init_packet() -> InitPacket {
    let mut vec = Vec::with_capacity(128);

    // TODO random IV
    for i in 0..127 {
        vec.push(i);
    }

    let packet = InitPacket {
        iv: vec,
        // TODO: now?
        timestamp: 0,
    };

    return packet;
}

/// handshake is sending the init packet
pub(crate) fn send_init_packet(
    stream: tokio::net::TcpStream,
) -> impl Future<Item = (u64, InitPacket, tokio::net::TcpStream), Error = Error> {
    let packet = new_init_packet();

    println!("sending init packet");

    copy(packet, stream)
}

impl InitPacket {
    pub(crate) fn encode(&self) -> [u8; 128 + 32] {
        let mut buf = [0; 128 + 32];
        for n in &self.iv {
            let _ = BigEndian::write_uint(&mut buf, (*n).into(), 1);
        }
        let _ = BigEndian::write_u32(&mut buf, self.timestamp);
        return buf;
    }
}

impl AsyncRead for InitPacket {}

impl Read for InitPacket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let write: usize = 0;
        for n in &self.iv {
            BigEndian::write_uint(buf, (*n).into(), 1);
        }
        BigEndian::write_u32(buf, self.timestamp);

        // TODO: crappy code
        Ok(128 + 32)
    }
}
