# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


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

### Limitations

- Only structs with named fields are supported.
- No regex support yet.
- No tracing integration yet.
- No serde integration yet.

## [Unreleased]

### Added

- Core `Redactor` type.
- `RedactorBuilder` for configuring redaction rules.
- Exact field-name redaction.
- Custom mask support.
- Unit tests for core redaction logic.
- `default_redactor()` with common sensitive field names.
- `Default` implementation for `Redactor`.

## [0.0.1] - Unreleased

Initial release is not published yet.
