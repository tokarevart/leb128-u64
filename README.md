# leb128-u64

Extremely minimal and simple [LEB128](https://en.wikipedia.org/wiki/LEB128) `u64` encoding/decoding.

[![Cargo](https://img.shields.io/crates/v/leb128-u64.svg)](https://crates.io/crates/leb128-u64)
[![Documentation](https://docs.rs/leb128-u64/badge.svg)](https://docs.rs/leb128-u64)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/tokarevart/leb128-u64)

## Example

```rust
let input: u64 = 42;
let mut buf = [0u8; 10];
leb128_u64::encode(input, &mut buf[..]);

let output = leb128_u64::decode(&buf[..]);
assert_eq!(input, output);
```
