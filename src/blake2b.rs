use std::hash::Hasher;
use std::io::Write;

use blake2_rfc::blake2b;

use {ByteHash, State};

/// Wrapping of `Blake2b` in `ByteHash`
#[derive(Clone, PartialEq, Eq)]
pub struct Blake2b;

pub struct Blake2bState {
    buf: [u8; 32],
    state: blake2b::Blake2b,
}

impl ByteHash for Blake2b {
    type Digest = [u8; 32];
    type State = Blake2bState;

    fn state() -> Blake2bState {
        Blake2bState {
            state: blake2b::Blake2b::new(32),
            buf: Default::default(),
        }
    }
}

impl State<[u8; 32]> for Blake2bState {
    fn fin(self) -> [u8; 32] {
        let Blake2bState { state, mut buf } = self;
        buf.as_mut()
            .write(state.finalize().as_bytes())
            .expect("in-memory write");
        buf
    }
}

impl Hasher for Blake2bState {
    fn write(&mut self, bytes: &[u8]) {
        self.state.write(bytes).expect("in-memory write");
    }

    fn finish(&self) -> u64 {
        panic!("Do not call `finish` on ByteHash, use `fin`")
    }
}
