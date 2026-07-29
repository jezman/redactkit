# redactkit-derive

[<img alt="github" src="https://img.shields.io/badge/github-jezman/redactkit-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/jezman/redactkit)
[<img alt="crates.io" src="https://img.shields.io/crates/v/redactkit-derive.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/redactkit-derive)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-redactkit-derive?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/redactkit-derive)

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
