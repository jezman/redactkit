# Security Policy

## Scope

`redactkit` helps reduce accidental leakage of sensitive data into output.

It does **not** guarantee secure memory erasure.

It does not protect against:

- memory dumps;
- swap;
- core dumps;
- secret copies made elsewhere in the program;
- unsafe code;
- side channels.

For secure secret handling, consider using:

- [`secrecy`](https://crates.io/crates/secrecy)
- [`zeroize`](https://crates.io/crates/zeroize)

## Reporting a Vulnerability

If you find a security issue, please do not open a public issue.

You can report it privately using GitHub Private Vulnerability Reporting

Please include:

- a description of the issue;
- steps to reproduce;
- affected version;
- potential impact.