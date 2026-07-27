//! # redactkit
//!
//! Small toolkit for redacting sensitive data in debug output and tracing logs.
//!
//! > **Warning:** `redactkit` redacts formatted output only.
//! > It does not modify or erase original field values in memory.
//!
//! ## Features
//!
//! | Feature | Default | Description |
//! | --- | --- | --- |
//! | `std` | yes | Standard library support. |
//! | `derive` | yes | Enables the `RedactDebug` derive macro. |
//! | `regex` | no | Enables regex-based field and value rules. |
//! | `tracing` | no | Enables `tracing-subscriber` field redaction. |
//!
//! ## Derive example
//!
//! ```
//! # #[cfg(feature = "derive")]
//! # {
//! use redactkit::RedactDebug;
//!
//! #[derive(RedactDebug)]
//! struct User {
//!     username: String,
//!     #[redact]
//!     password: String,
//! }
//!
//! let user = User {
//!     username: "anna".to_string(),
//!     password: "s3cr3t".to_string(),
//! };
//!
//! assert_eq!(
//!     format!("{user:?}"),
//!     "User { username: \"anna\", password: \"******\" }"
//! );
//! # }
//! ```
//!
//! ## Builder example
//!
//! ```
//! use redactkit::Redactor;
//!
//! let redactor = Redactor::builder()
//!     .field("password")
//!     .field("token")
//!     .mask("[hidden]")
//!     .build();
//!
//! assert_eq!(redactor.redact_field("password", "secret"), "[hidden]");
//! assert_eq!(redactor.redact_field("username", "anna"), "anna");
//! ```
//!
//! ## Default redactor
//!
//! ```
//! let redactor = redactkit::default_redactor();
//!
//! assert_eq!(redactor.redact_field("password", "secret"), "******");
//! assert_eq!(redactor.redact_field("api_key", "abc123"), "******");
//! ```
//!
//! ## Regex rules
//!
//! ```
//! # #[cfg(feature = "regex")]
//! # {
//! use redactkit::Redactor;
//!
//! let redactor = Redactor::builder()
//!     .field_pattern("(?i)token|secret")
//!     .unwrap()
//!     .value_pattern(r"\d{4}", "****")
//!     .unwrap()
//!     .build();
//!
//! assert_eq!(redactor.redact_field("api_token", "abcd"), "******");
//! assert_eq!(redactor.redact_field("note", "card 1234"), "card ****");
//! # }
//! ```
//!
//! ## Tracing integration
//!
//! ```
//! # #[cfg(feature = "tracing")]
//! # {
//! use tracing_subscriber::fmt;
//!
//! fmt()
//!     .fmt_fields(redactkit::tracing::redact_fields())
//!     .init();
//! # }
//! ```

#![forbid(unsafe_code)]

pub mod builder;
pub mod error;
pub mod patterns;
pub mod redactor;

#[cfg(feature = "tracing")]
pub mod tracing;

pub use builder::RedactorBuilder;
pub use error::Error;
pub use redactor::{Redactor, default_redactor};

#[cfg(feature = "derive")]
pub use redactkit_derive::RedactDebug;
