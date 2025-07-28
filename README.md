# one-based

[![CI](https://github.com/xkikeg/one-based/actions/workflows/ci.yml/badge.svg)](https://github.com/xkikeg/one-based/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/xkikeg/one-based/graph/badge.svg?token=QOAN4RFSA6)](https://codecov.io/gh/xkikeg/one-based)

This crate provides simple unsigned wrappers to handle 1-based index.

```rust
let v1 = OneBasedU32::from_one_based(1).uwrap();
assert_eq!(v1.as_zero_based(), 0);

let v2 = OneBasedU32::from_zero_based(0).uwrap();
assert_eq!(v2.as_one_based().get(), 1);

assert_eq!(v1, v2);
```
