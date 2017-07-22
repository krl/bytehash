# Cryptohash

Trait abstracting over cryptographic hash-functions

[Documentation](https://krl.github.io/rustdoc/cryptohash/cryptohash/index.html)

# Example

```rust
use cryptohash::{Blake2b, CryptoHash, CryptoHashState};

fn main() {
    let mut state = Blake2b::state();
    state.write(b"hello world").unwrap();
		
    assert_eq!(
    format!("{}", state.fin()),
            "256c83b297114d201b30179f3f0ef0cace9783622da5974326b436178aeef610"
    );
}

```