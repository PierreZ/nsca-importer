use byteorder::{BigEndian, ByteOrder};
pub struct NSCACodec;

struct InitPacket {
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

impl InitPacket {
    fn encode(&self) -> [u8; 128 + 32] {
        let mut buf = [0; 128 + 32];
        for n in &self.iv {
            let _ = BigEndian::write_uint(&mut buf, (*n).into(), 1);
        }
        let _ = BigEndian::write_u32(&mut buf, self.timestamp);
        return buf;
    }
}
