# redactkit

[![Crates.io](https://img.shields.io/crates/v/redactkit.svg)](https://crates.io/crates/redactkit)
[![Crates.io](https://img.shields.io/crates/d/redactkit.svg)](https://crates.io/crates/redactkit)
[![Docs.rs](https://docs.rs/redactkit/badge.svg)](https://docs.rs/redactkit)
[![License](https://img.shields.io/crates/l/redactkit.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

Keep secrets out of your logs.

`redactkit` helps prevent accidental leakage of passwords, tokens, API keys,
and other sensitive data into debug output, logs, and serialized data.

## Warning

This crate focuses on redaction of output.

It does **not** provide secure memory erasure.

For secure secret handling, consider using:

- [`secrecy`](https://crates.io/crates/secrecy)
- [`zeroize`](https://crates.io/crates/zeroize)

## Status

This is an early preview.

Current status:

- [x] Core `Redactor`
- [x] Builder API
- [x] Exact field-name redaction
- [x] Custom mask support
- [x] Default redactor
- [x] Derive macro
- [x] Regex rules
- [ ] `tracing` integration
- [ ] `serde` helpers

## Derive

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

## Builder

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .field("token")
    .build();

assert!(redactor.should_redact_field("password"));
assert!(redactor.should_redact_field("token"));
assert!(!redactor.should_redact_field("username"));

assert_eq!(redactor.redact_field("password", "s3cr3t"), "******");
assert_eq!(redactor.redact_field("username", "anna"), "anna");
```

## Custom mask

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .mask("[REDACTED]")
    .build();

assert_eq!(redactor.redact_field("password", "s3cr3t"), "[REDACTED]");
```

## Default redactor

```rust
use redactkit::default_redactor;

let redactor = default_redactor();

assert!(redactor.should_redact_field("password"));
assert!(redactor.should_redact_field("token"));
assert!(redactor.should_redact_field("api_key"));
assert!(!redactor.should_redact_field("username"));
```

## Features

`redactkit` uses optional feature flags.

| Feature  | Default | Description                                |
| -------- | ------- | ------------------------------------------ |
| `std`    | yes     | Standard library support.                  |
| `derive` | yes     | Enables the `RedactDebug` derive macro.    |
| `regex`  | no      | Enables regex-based field and value rules. |

## Regex rules

Enable the `regex` feature:

```toml
[dependencies]
redactkit = { version = "0.0.2", features = ["regex"] }
```

Then use regex-based rules:

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .field_pattern("(?i)token|secret")
    .unwrap()
    .value_pattern(r"\d{4}", "****")
    .unwrap()
    .build();

assert!(redactor.should_redact_field("password"));
assert!(redactor.should_redact_field("api_token"));
assert!(redactor.should_redact_field("CLIENT_SECRET"));
assert!(!redactor.should_redact_field("username"));

assert_eq!(
    redactor.redact_field("note", "card 1234 ok"),
    "card **** ok"
);
```

`field_pattern` matches against field names.

`value_pattern` replaces matched parts of the value.

Both methods return `Result`, because an invalid regex pattern can fail to compile:

```rust
use redactkit::Redactor;

let result = Redactor::builder()
    .field_pattern("(unclosed");

assert!(result.is_err());
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
