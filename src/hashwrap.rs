use std::hash::Hasher;
use std::marker::PhantomData;

use byteorder::{BigEndian, WriteBytesExt};

use {ByteHash, State};

/// Wrapping any `Hasher` in ByteHash
#[derive(Clone, Default)]
pub struct Wrapped<H>(PhantomData<H>);

/// Wrapped state for computing hashes
pub struct WrappedState<H> {
    buf: [u8; 8],
    state: H,
}

impl<H: Hasher> Hasher for WrappedState<H> {
    fn write(&mut self, bytes: &[u8]) {
        self.state.write(bytes)
    }

    fn finish(&self) -> u64 {
        panic!("Do not call `finish` on ByteHash, use `fin`")
    }
}

impl<H> ByteHash for Wrapped<H>
where
    H: 'static + Hasher + Default + Clone,
{
    type Digest = [u8; 8];
    type State = WrappedState<H>;

    fn state() -> Self::State {
        WrappedState {
            buf: [0u8; 8],
            state: H::default(),
        }
    }
}

impl<H> State<[u8; 8]> for WrappedState<H>
where
    H: Hasher,
{
    fn fin(self) -> [u8; 8] {
        let WrappedState { state, mut buf } = self;
        buf.as_mut()
            .write_u64::<BigEndian>(state.finish())
            .expect("in-memory write");
        buf
    }
}
