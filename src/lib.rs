#![deny(missing_docs)]
//! Trait abstracting over hash-functions, allowing digests to be viewed as
//! byte slices.
extern crate blake2_rfc;
extern crate byteorder;

use std::io::Write;

mod blake2b;
mod hashwrap;

pub use blake2b::Blake2b;
pub use hashwrap::Wrapped;

/// A wrapper trait for cryptographic hash functions
pub trait ByteHash: Write {
    /// The type that is used for the final hash value
    type Digest: AsRef<[u8]> + AsMut<[u8]>;
    /// Consumes the `ByteHash` and returns a `Digest`
    fn fin(self) -> Self::Digest;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashwrap::Wrapped;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn default() {
        let mut state = Wrapped::<DefaultHasher>::default();
        state.write(b"hello world").unwrap();
        state.fin();
    }

    #[test]
    fn blake2b() {
        let mut state = Blake2b::default();
        state.write(b"hello world").unwrap();
        state.fin();
    }
}
