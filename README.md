# redactkit

[![Crates.io](https://img.shields.io/crates/v/redactkit.svg)](https://crates.io/crates/redactkit)
[![Downloads](https://img.shields.io/crates/d/redactkit.svg)](https://crates.io/crates/redactkit)
[![Docs](https://docs.rs/redactkit/badge.svg)](https://docs.rs/redactkit)
[![License](https://img.shields.io/crates/l/redactkit.svg)](#license)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org/)

Keep secrets out of your logs.

`redactkit` helps prevent accidental leakage of passwords, tokens, API keys,
and other sensitive data into debug output, logs, and serialized data.

> ## Warning
>
> This crate focuses on redaction of output.
> It does not provide secure memory erasure.
>
> For secure secret handling, consider using:
>
> - [`secrecy`](https://crates.io/crates/secrecy)
> - [`zeroize`](https://crates.io/crates/zeroize)

## Examples

### Derive

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

### Builder

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

### Custom mask

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .mask("[REDACTED]")
    .build();

assert_eq!(redactor.redact_field("password", "s3cr3t"), "[REDACTED]");
```

### Default redactor

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

| Feature   | Default | Description                                   |
| --------- | ------- | --------------------------------------------- |
| `std`     | yes     | Standard library support.                     |
| `derive`  | yes     | Enables the `RedactDebug` derive macro.       |
| `regex`   | no      | Enables regex-based field and value rules.    |
| `tracing` | no      | Enables `tracing-subscriber` field redaction. |
| `serde`   | no      | Enables the `RedactSerialize` derive macro for redacting fields during serialization.   |


## Regex rules

Enable the `regex` feature:

```toml
[dependencies]
redactkit = { version = "0.2", features = ["regex"] }
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

## Tracing integration

Enable the `tracing` feature:

```toml
[dependencies]
redactkit = { version = "0.2", features = ["tracing"] }
```

Then configure `tracing-subscriber` to use the redacting field formatter:

```rust
use tracing_subscriber::fmt;

fmt()
    .fmt_fields(redactkit::tracing::redact_fields())
    .init();

tracing::info!(user = "anna", password = "s3cr3t");
```

The log output will contain:

```text
user="anna" password="******"
```

You can add custom sensitive fields:

```rust
use tracing_subscriber::fmt;

fmt()
    .fmt_fields(
        redactkit::tracing::redact_fields()
            .field("session_id")
            .field("cookie"),
    )
    .init();
```

With the `regex` feature enabled, you can also use patterns:

```rust
use tracing_subscriber::fmt;

fmt()
    .fmt_fields(
        redactkit::tracing::redact_fields()
            .field_pattern("(?i)token|secret")
            .unwrap(),
    )
    .init();
```

The tracing integration redacts formatted output only.
It does not modify or erase original field values in memory.

## Serde integration

`redactkit` can also redact sensitive fields during serialization.

Enable the `serde` feature:

```toml
[dependencies]
redactkit = { version = "0.2", features = ["serde"] }
```

Then derive `RedactSerialize`:

```rust
use redactkit::RedactSerialize;

#[derive(RedactSerialize)]
struct Config {
    username: String,
    #[redact]
    password: String,
}
```

When serialized, redacted fields are replaced with a mask:

```json
{
  "username": "anna",
  "password": "******"
}
```

The original values in memory are not modified.

### Serde limitations

Currently `RedactSerialize`:

- supports only structs with named fields;
- does not support generics yet;
- serializes redacted fields as the mask string `"******"`;
- may change the serialized type of a redacted field.

For example, if a redacted field was originally an integer, it will be serialized as a string mask.

## Motivation

Accidental secret leakage in logs is a common security incident.

A single `tracing::debug!` or `println!("{:?}", user)` can expose
passwords, tokens, API keys, and session identifiers.

`redactkit` provides a small, explicit toolkit for redacting such
values at formatting time, so that sensitive fields are replaced with
a mask before they reach logs, debug output, or error messages.

## When to use redactkit

Use `redactkit` when you need to:

- redact sensitive fields in `Debug` output of structs;
- redact fields in `tracing` log output;
- redact fields during serialization;
- build a custom redaction policy with exact field names or regex rules;
- keep the original values intact in memory and only redact formatted output.

## When NOT to use redactkit

`redactkit` is not a memory-protection crate.

It does not:

- erase secrets from memory;
- prevent access to original field values;
- encrypt data at rest or in transit;
- replace proper secret management.

If you need to wrap secrets and control their lifetime, consider
combining `redactkit` with a dedicated crate such as `secrecy`.

## Comparison with `secrecy`

`secrecy` and `redactkit` solve related but different problems.

| Crate       | Primary focus               | How it works                                                                                       |
| ----------- | --------------------------- | -------------------------------------------------------------------------------------------------- |
| `secrecy`   | Secret storage and lifetime | Wraps a secret value in `Secret<T>` and prevents accidental `Debug` / `Display` leakage.           |
| `redactkit` | Output redaction            | Redacts selected fields when formatting output, while original values remain accessible in memory. |

A typical split:

- use `secrecy` when you want to wrap a secret value itself;
- use `redactkit` when you want to redact selected fields in logs, debug output, tracing output, or serialized output.

They can also be used together: `secrecy` protects the value,
while `redactkit` helps enforce redaction policies for formatted output.

## Status

Current feature set:

- core `Redactor`
- builder API
- exact field-name redaction
- custom mask support
- default redactor
- `RedactDebug` derive macro
- `RedactSerialize` derive macro
- regex rules
- `tracing` integration
- examples
- CI quality gates

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>