//! # redactkit
//!
//! `redactkit` helps prevent accidental leakage of passwords, tokens,
//! API keys, and other sensitive data into debug output, logs,
//! and serialized data.
//!
//! This first milestone implements only the core redaction logic.

#![forbid(unsafe_code)]

pub mod builder;
pub mod redactor;

pub use builder::RedactorBuilder;
pub use redactor::Redactor;
