//! # redactkit
//!
//! `redactkit` helps prevent accidental leakage of passwords, tokens,
//! API keys, and other sensitive data into debug output, logs,
//! and serialized data.
//!
//! This first milestone implements only the core redaction logic.

#![forbid(unsafe_code)]

pub mod builder;
pub mod patterns;
pub mod redactor;

pub use builder::RedactorBuilder;
pub use redactor::Redactor;

#[cfg(feature = "derive")]
pub use redactkit_derive::RedactDebug;

use crate::patterns::DEFAULT_SENSITIVE_FIELDS;

/// Returns a redactor with common default sensitive field names.
///
/// # Examples
///
/// ```
/// use redactkit::default_redactor;
///
/// let redactor = default_redactor();
///
/// assert!(redactor.should_redact_field("password"));
/// assert!(redactor.should_redact_field("token"));
/// assert!(redactor.should_redact_field("api_key"));
/// assert!(!redactor.should_redact_field("username"));
/// ```
pub fn default_redactor() -> Redactor {
    Redactor::builder()
        .fields(DEFAULT_SENSITIVE_FIELDS.iter().copied())
        .build()
}

impl Default for Redactor {
    fn default() -> Self {
        default_redactor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_redactor_redacts_common_fields() {
        let redactor = default_redactor();

        assert!(redactor.should_redact_field("password"));
        assert!(redactor.should_redact_field("passwd"));
        assert!(redactor.should_redact_field("secret"));
        assert!(redactor.should_redact_field("token"));
        assert!(redactor.should_redact_field("access_token"));
        assert!(redactor.should_redact_field("refresh_token"));
        assert!(redactor.should_redact_field("api_key"));
        assert!(redactor.should_redact_field("apikey"));
        assert!(redactor.should_redact_field("authorization"));
        assert!(redactor.should_redact_field("private_key"));
        assert!(redactor.should_redact_field("client_secret"));
        assert!(redactor.should_redact_field("database_url"));
    }

    #[test]
    fn default_redactor_does_not_redact_random_fields() {
        let redactor = default_redactor();

        assert!(!redactor.should_redact_field("username"));
        assert!(!redactor.should_redact_field("email"));
        assert!(!redactor.should_redact_field("host"));
        assert!(!redactor.should_redact_field("port"));
    }

    #[test]
    fn default_redactor_masks_values() {
        let redactor = default_redactor();

        assert_eq!(redactor.redact_field("password", "hunter2"), "******");

        assert_eq!(redactor.redact_field("username", "alice"), "alice");
    }

    #[test]
    fn redactor_default_is_same_as_default_redactor() {
        let from_default = Redactor::default();
        let from_function = default_redactor();

        assert!(from_default.should_redact_field("password"));
        assert!(from_function.should_redact_field("password"));
    }
}
