use std::fmt::{Display, Formatter};
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use crate::registry::IResult;

pub const MAGIC: u8 = 0xFE;

pub struct ByteBuffer {
    pub buf: Vec<u8>,
    pub pos: usize,
}


impl ByteBuffer {
    pub fn new() -> Self {
        Self {
            buf: vec![0; 1024],
            pos: 0,
        }
    }

    pub fn get_packet(&self) -> IResult<Vec<Packet>> {
        let mut packets: Vec<Packet> = vec![];

        let mut reader = &self.buf[..];
        loop {
            let mut packet = Packet::new();
            packet.magic = reader.read_u8()?;
            if packet.magic != MAGIC {
                break;
            }

            packet.len = reader.read_u8()?;
            packet.cmd = reader.read_u8()?;

            let body_len = packet.len as usize;
            let mut body = vec![0u8; body_len];
            reader.read_exact(&mut body)?;

            packet.body = body;

            packets.push(packet)
        }
        Ok(packets)
    }
}


#[derive(Debug, Clone)]
pub struct Packet {
    pub magic: u8,
    pub len: u8,
    pub cmd: u8,
    pub body: Vec<u8>,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            magic: 0,
            len: 0,
            cmd: 0,
            body: vec![],
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "magic: {:?}, len: {:?}, cmd: {:?}, body: {:?}", self.magic, self.len, self.cmd, self.body)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data() {}
}