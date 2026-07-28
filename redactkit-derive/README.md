# redactkit-derive

[![Crates.io](https://img.shields.io/crates/v/redactkit-derive.svg)](https://crates.io/crates/redactkit-derive)
[![Docs](https://docs.rs/redactkit-derive/badge.svg)](https://docs.rs/redactkit-derive)
[![License](https://img.shields.io/crates/l/redactkit-derive.svg)](#license)

Derive macros for [`redactkit`](https://crates.io/crates/redactkit).

This crate provides:

- `RedactDebug` — derives a safe `Debug` implementation;
- `RedactSerialize` — derives a redacting `serde::Serialize` implementation.

Users normally should not depend on this crate directly.
Use [`redactkit`](https://crates.io/crates/redactkit) instead.

## Example

```rust
use redactkit::RedactDebug;

#[derive(RedactDebug)]
struct Config {
    username: String,
    #[redact]
    password: String,
}

let config = Config {
    username: "anna".to_string(),
    password: "s3cr3t".to_string(),
};

let debug = format!("{config:?}");

assert!(debug.contains("anna"));
assert!(debug.contains("******"));
assert!(!debug.contains("s3cr3t"));
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>