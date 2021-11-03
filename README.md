![MIT licensed](https://img.shields.io/github/license/dedefer/serde_bytes_wrapper?style=for-the-badge)
[![Version](https://img.shields.io/crates/v/serde_bytes_wrapper?style=for-the-badge)](https://crates.io/crates/serde_bytes_wrapper/)
![Code Coverage](https://img.shields.io/coveralls/github/dedefer/serde_bytes_wrapper/main?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/serde_bytes_wrapper?style=for-the-badge)

# serde_bytes_wrapper

Wrapper for Vec<u8>, which uses serde_bytes as representation.

It implements Deserialize, Serialize and Deref/DerefMut to Vec<u8>;

[Documentation link](https://docs.rs/serde_bytes_wrapper/)

[Crates.io link](https://crates.io/crates/serde_bytes_wrapper/)

It is useful when you want something like
```rust
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Val {
    #[serde(with = "serde_bytes")]
    val: Option<Vec<Vec<u8>>>,
}
```
you can use instead
```rust
use serde_bytes_wrapper::Bytes;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Val {
    val: Option<Vec<Bytes>>,
}
```

## Example

```rust
use serde::{Deserialize, Serialize};
use serde_bytes_wrapper::Bytes;

#[derive(Deserialize, Serialize, Debug)]
struct Val {
    val: Option<Vec<Bytes>>,
}

fn main() {
  let result = serde_cbor::to_vec(&Val {
      val: Some(vec![vec![1, 2, 3].into()])
    }).unwrap();
  println!("{:?}", result); // [161, 99, 118, 97, 108, 129, 67, 1, 2, 3]
}
```
