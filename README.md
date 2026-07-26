# redactkit

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

Current status:

- [x] Core `Redactor`
- [x] Builder API
- [x] Exact field-name redaction
- [x] Custom mask support
- [X] Default redactor
- [ ] Derive macro
- [ ] Regex rules
- [ ] `tracing` integration
- [ ] `serde` helpers

## Example

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .field("token")
    .build();

assert!(redactor.should_redact_field("password"));
assert!(redactor.should_redact_field("token"));
assert!(!redactor.should_redact_field("username"));

assert_eq!(
    redactor.redact_field("password", "hunter2"),
    "******"
);

assert_eq!(
    redactor.redact_field("username", "alice"),
    "alice"
);
```

## Custom mask

```rust
use redactkit::Redactor;

let redactor = Redactor::builder()
    .field("password")
    .mask("[hidden]")
    .build();

assert_eq!(
    redactor.redact_field("password", "hunter2"),
    "[hidden]"
);
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

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>