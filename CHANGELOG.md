# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.3]

### Added

- Optional `tracing` feature.
- `redactkit::tracing::redact_fields()` formatter for `tracing-subscriber`.
- `RedactFields` builder methods:
  - `field`
  - `fields`
  - `mask`
  - `field_pattern` (requires `regex`)
  - `value_pattern` (requires `regex`)
- Tracing integration tests.
- Tracing example `examples/tracing_fmt.rs`.

### Notes

- The tracing integration redacts formatted output only.
  It does not modify or erase original field values in memory.

## [0.0.2]

### Added

- Optional `regex` feature.
- `Error::InvalidRegex` typed error.
- `RedactorBuilder::field_pattern` for matching field names by regex.
- `RedactorBuilder::value_pattern` for replacing matched value parts.
- Compiled regex patterns are stored inside `Redactor`.

## [0.0.1]

Initial early preview release.

### Added

- `#[redact]` attribute support for struct fields.
- `RedactDebug` derive macro.
- Core `Redactor` type.
- `RedactorBuilder` for configuring redaction rules.
- Exact field-name redaction.
- Custom mask support.
- Unit tests for core redaction logic.
- `default_redactor()` with common sensitive field names.
- `Default` implementation for `Redactor`.

### Limitations

- Only structs with named fields are supported.
- No regex support yet.
- No tracing integration yet.
- No serde integration yet.

[0.0.3]: https://github.com/jezman/redactkit/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jezman/redactkit/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jezman/redactkit/releases/tag/v0.0.1
