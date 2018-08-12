use byteorder::{LittleEndian, WriteBytesExt};
use std::hash::Hasher;
use std::io::{self, Write};

use ByteHash;

/// Wrapping any `Hasher` in ByteHash
#[derive(Default)]
pub struct Wrapped<H> {
    buf: [u8; 8],
    state: H,
}

impl<H: Hasher> Write for Wrapped<H> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.state.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<H> ByteHash for Wrapped<H>
where
    H: Hasher,
{
    type Digest = [u8; 8];

    fn fin(self) -> Self::Digest {
        let Wrapped { state, mut buf } = self;
        buf.as_mut()
            .write_u64::<LittleEndian>(state.finish())
            .expect("in-memory write");
        buf
    }
}
