/*!
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
*/

use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub struct Bytes(#[serde(with = "serde_bytes")] Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(val: Vec<u8>) -> Self { Bytes(val) }
}

impl From<Bytes> for Vec<u8> {
    fn from(Bytes(val): Bytes) -> Self { val }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use serde_bytes_repr::{ByteFmtDeserializer, ByteFmtSerializer};
    use crate::Bytes;



    #[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
    struct Val {
        val: Option<Vec<Bytes>>,
    }

    #[test]
    fn test_serialize_json() {
        let mut raw_res: Vec<u8> = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut raw_res);
        let ser = ByteFmtSerializer::base64(&mut ser, base64::STANDARD);
        Val { val: Some(vec![vec![1, 2 ,3].into()]) }.serialize(ser).unwrap();

        let result = String::from_utf8(raw_res).unwrap();
        assert_eq!(result, r#"{"val":["AQID"]}"#.to_owned());
    }

    #[test]
    fn test_deserialize_json() {
        let mut de = serde_json::Deserializer::from_str(r#"{"val":["AQID"]}"#);
        let de = ByteFmtDeserializer::new_base64(&mut de, base64::STANDARD);
        let result: Val = Deserialize::deserialize(de).unwrap();
        assert_eq!(result, Val { val: Some(vec![vec![1, 2 ,3].into()]) });
    }

    #[test]
    fn test_deref() {
        let mut result: Bytes = vec![1, 2, 3].into();
        for byte in result.iter_mut() {
            *byte = 100
        }
        result.push(100);
        let sum = result.iter().fold(0u32, |sum, &el| sum + el as u32);
        let v: Vec<u8> = result.into();
        assert_eq!((v, sum), (vec![100, 100, 100, 100], 400));
    }

    #[test]
    fn test_serialize_cbor() {
        let result = serde_cbor::to_vec(&Val { val: Some(vec![vec![1, 2 ,3].into()]) }).unwrap();
        assert_eq!(result, vec![161, 99, 118, 97, 108, 129, 67, 1, 2, 3]);
    }

    #[test]
    fn test_deserialize_cbor() {
        let result: Val = serde_cbor::from_slice(&[161, 99, 118, 97, 108, 129, 67, 1, 2, 3]).unwrap();
        assert_eq!(result, Val { val: Some(vec![vec![1, 2 ,3].into()]) });
    }
}
