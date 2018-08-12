# ByteHash

Trait abstracting over hash-functions, allowing digests to be viewed as byte slices.

This allows you to use cryptographic and non-cryptographic hash functions interchangeably.

[Documentation](https://docs.rs/cryptohash/1.0.0/bytehash/)

# Example

```rust
use bytehash::{Blake2b, ByteHash, Wrapped};
use std::collections::hash_map::DefaultHasher;

fn main() {
    let mut blake = Blake2b::default();
    blake.write(b"hello world").unwrap();
    let hash = blake.fin();

    let mut default = Wrapped::<DefaultHasher>::default();
    default.write(b"hello world").unwrap();
    let hash = default.fin();
}
```