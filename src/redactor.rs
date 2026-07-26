use crate::builder::RedactorBuilder;

/// Redacts sensitive fields and values.
///
/// # Examples
///
/// ```
/// use redactkit::Redactor;
///
/// let redactor = Redactor::builder()
///     .field("password")
///     .build();
///
/// assert_eq!(
///     redactor.redact_field("password", "hunter2"),
///     "******"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Redactor {
    fields: Vec<String>,
    mask: String,
}

impl Redactor {
    /// Creates a new [`RedactorBuilder`].
    pub fn builder() -> RedactorBuilder {
        RedactorBuilder::new()
    }

    /// Internal constructor used by the builder.
    ///
    /// Мы не делаем его публичным, потому что пользователь должен
    /// создавать `Redactor` через builder.
    pub(crate) fn from_parts(fields: Vec<String>, mask: String) -> Self {
        Self { fields, mask }
    }

    /// Returns `true` if the given field name should be redacted.
    ///
    /// Matching is currently exact and case-sensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .field("password")
    ///     .build();
    ///
    /// assert!(redactor.should_redact_field("password"));
    /// assert!(!redactor.should_redact_field("username"));
    /// ```
    pub fn should_redact_field(&self, field: &str) -> bool {
        self.fields.iter().any(|name| name == field)
    }

    /// Redacts the value if the field is sensitive.
    ///
    /// If the field is sensitive, returns the mask.
    /// Otherwise returns the original value.
    ///
    /// # Examples
    ///
    /// ```
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .field("password")
    ///     .build();
    ///
    /// assert_eq!(
    ///     redactor.redact_field("password", "hunter2"),
    ///     "******"
    /// );
    ///
    /// assert_eq!(
    ///     redactor.redact_field("username", "alice"),
    ///     "alice"
    /// );
    /// ```
    pub fn redact_field(&self, field: &str, value: &str) -> String {
        if self.should_redact_field(field) {
            self.mask.clone()
        } else {
            value.to_string()
        }
    }

    /// Redacts a value unconditionally.
    ///
    /// The original value is ignored and the configured mask is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .field("password")
    ///     .build();
    ///
    /// assert_eq!(redactor.redact_value("hunter2"), "******");
    /// ```
    pub fn redact_value(&self, _value: &str) -> String {
        self.mask.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_exact_field_name() {
        let redactor = Redactor::builder().field("password").build();

        assert!(redactor.should_redact_field("password"));
        assert!(!redactor.should_redact_field("username"));
    }

    #[test]
    fn redact_field_returns_mask_for_sensitive_field() {
        let redactor = Redactor::builder().field("password").build();

        assert_eq!(redactor.redact_field("password", "hunter2"), "******");
    }

    #[test]
    fn redact_field_returns_original_for_non_sensitive_field() {
        let redactor = Redactor::builder().field("password").build();

        assert_eq!(redactor.redact_field("username", "alice"), "alice");
    }

    #[test]
    fn supports_multiple_fields() {
        let redactor = Redactor::builder().fields(["password", "token"]).build();

        assert!(redactor.should_redact_field("password"));
        assert!(redactor.should_redact_field("token"));
        assert!(!redactor.should_redact_field("username"));
    }

    #[test]
    fn custom_mask_can_be_used() {
        let redactor = Redactor::builder()
            .field("password")
            .mask("[hidden]")
            .build();

        assert_eq!(redactor.redact_field("password", "hunter2"), "[hidden]");
    }

    #[test]
    fn redact_value_always_returns_mask() {
        let redactor = Redactor::builder().field("password").build();

        assert_eq!(redactor.redact_value("anything"), "******");
    }
    
    #[test]
    fn field_matching_is_case_sensitive_for_now() {
        let redactor = Redactor::builder()
            .field("password")
            .build();
    
        assert!(redactor.should_redact_field("password"));
        assert!(!redactor.should_redact_field("PASSWORD"));
        assert!(!redactor.should_redact_field("Password"));
    }
}
