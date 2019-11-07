use std::fmt;
use std::hash::Hasher;
use std::io::{self, Write};
use std::marker::PhantomData;

use byteorder::{LittleEndian, WriteBytesExt};

use {ByteHash, State};

/// Wrapping any `Hasher` in ByteHash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapped<H>(PhantomData<H>);

/// Wrapped state for computing hashes
#[derive(Default)]
pub struct WrappedState<H>
where
    H: Default,
{
    buf: [u8; 8],
    state: H,
}

impl<H: Hasher> Write for WrappedState<H>
where
    H: Default,
{
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
    H: 'static + Hasher + Default + Clone + fmt::Debug + Eq,
{
    type Digest = [u8; 8];
    type State = WrappedState<H>;

    fn state() -> Self::State {
        WrappedState::default()
    }
}

impl<H> State<[u8; 8]> for WrappedState<H>
where
    H: Hasher + Default,
{
    fn fin(self) -> [u8; 8] {
        let WrappedState { state, mut buf } = self;
        buf.as_mut()
            .write_u64::<LittleEndian>(state.finish())
            .expect("in-memory write");
        buf
    }
}
