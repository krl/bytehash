use std::mem;
use std::fmt;

use blake2_rfc::blake2b;

use {CryptoHash, CryptoHashState};

/// Wraps Blake2b in the `CryptoHash` trait
#[derive(Clone)]
pub struct Blake2b {}

impl CryptoHash for Blake2b {
    type State = blake2b::Blake2b;
    type Digest = BlakeDigestWrap;

    fn state() -> Self::State {
        blake2b::Blake2b::new(32)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct BlakeDigestWrap([u8; 32]);

impl CryptoHashState<BlakeDigestWrap> for blake2b::Blake2b {
    fn fin(self) -> BlakeDigestWrap {
        let mut bytes: [u8; 32];

        let fin = self.finalize();
        let resultbytes = fin.as_bytes();

        // this is safe since we always fill the buffer completely.
        unsafe { bytes = mem::uninitialized() }
        for i in 0..bytes.len() {
            bytes[i] = resultbytes[i]
        }
        BlakeDigestWrap(bytes)
    }
}

impl AsRef<[u8]> for BlakeDigestWrap {
    fn as_ref<'a>(&'a self) -> &'a [u8] {
        &self.0
    }
}

impl fmt::Display for BlakeDigestWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.as_ref() {
            write!(f, "{:02x}", i)?;
        }
        Ok(())
    }
}
