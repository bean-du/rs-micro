use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crate::registry::IResult;


pub struct ByteBuffer {
    pub buf: [u8; 1024],
    pub pos: usize
}

impl ByteBuffer {
    pub fn new() -> Self {
        Self{
            buf: [0; 1024],
            pos: 0,
        }
    }

    pub fn read_u8(&mut self) -> IResult<u8> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }

        let mut s = Cursor::new(self.buf);
        let res = s.read_u8()?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buf () {
        let mut  bb = ByteBuffer::new();
        bb.buf[0] = 0xfe;
    }
}