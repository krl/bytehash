//! Trait abstracting over cryptographic hash-functions.
extern crate blake2_rfc;

use std::io::Write;
use std::hash::Hash;

mod blake2b;

pub use blake2b::Blake2b;

/// The trait for a hash state
pub trait State<Digest>
where
    Digest: AsRef<[u8]>,
{
    fn fin(self) -> Digest;
}

/// A wrapper trait for cryptographic hash functions
pub trait CryptoHash
where
    Self: 'static + Clone,
{
    /// The output type of the hash function
    type Digest: AsRef<[u8]>
        + AsMut<[u8]>
        + Copy
        + Hash
        + Ord
        + Eq
        + std::fmt::Display;
    /// The hash-state currently being computed
    type State: Write + State<Self::Digest>;
    /// Constructor for a new hash-state
    fn state() -> Self::State;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use blake2b::Blake2b;

    #[test]
    fn blake() {
        let mut state = Blake2b::state();
        state.write(b"hello world").unwrap();

        assert_eq!(
            format!("{}", state.fin()),
            "256c83b297114d201b30179f3f0ef0cace9783622da5974326b436178aeef610"
        );
    }
}
