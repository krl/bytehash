#![deny(missing_docs)]
//! Trait abstracting over hash-functions, allowing digests to be viewed as
//! byte slices.
extern crate blake2_rfc;
extern crate byteorder;

use std::fmt;
use std::hash::Hash;
use std::io::Write;

mod blake2b;
mod hashwrap;

pub use blake2b::Blake2b;
pub use hashwrap::Wrapped;

/// The main trait
pub trait ByteHash: 'static + Clone + fmt::Debug {
    /// The type that is used for the final hash value
    type Digest: AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Clone
        + Eq
        + Hash
        + Default
        + fmt::Debug
        + Send;

    /// The state for computing hashes
    type State: State<Self::Digest> + Write;

    /// Construct a new hash-state
    fn state() -> Self::State;
}

/// A hash state being computed
pub trait State<D> {
    /// Consumes the state returning a hash
    fn fin(self) -> D;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashwrap::Wrapped;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn default() {
        let mut state = Wrapped::<DefaultHasher>::state();
        state.write(b"hello world").unwrap();
        state.fin();
    }

    #[test]
    fn blake2b() {
        let mut state = Blake2b::state();
        state.write(b"hello world").unwrap();
        state.fin();
    }
}
