use std::io::{self, Write};

use blake2_rfc::blake2b;

use ByteHash;

/// Wrapping of `Blake2b` in `ByteHash`
pub struct Blake2b {
    buf: [u8; 32],
    state: blake2b::Blake2b,
}

impl Default for Blake2b {
    fn default() -> Self {
        Blake2b {
            state: blake2b::Blake2b::new(32),
            buf: Default::default(),
        }
    }
}

impl ByteHash for Blake2b {
    type Digest = [u8; 32];

    fn fin(self) -> Self::Digest {
        let Blake2b { state, mut buf } = self;
        buf.as_mut()
            .write(state.finalize().as_bytes())
            .expect("in-memory write");
        buf
    }
}

impl Write for Blake2b {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.state.write(buf).expect("in-memory write");
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
