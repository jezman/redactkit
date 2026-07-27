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
///     redactor.redact_field("password", "s3cr3t"),
///     "******"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Redactor {
    pub(crate) fields: Vec<String>,
    pub(crate) mask: String,

    #[cfg(feature = "regex")]
    pub(crate) field_patterns: Vec<regex::Regex>,

    #[cfg(feature = "regex")]
    pub(crate) value_patterns: Vec<(regex::Regex, String)>,
}

impl Redactor {
    /// Creates a new [`RedactorBuilder`].
    pub fn builder() -> RedactorBuilder {
        RedactorBuilder::new()
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
        if self.fields.iter().any(|name| name == field) {
            return true;
        }

        #[cfg(feature = "regex")]
        {
            if self.field_patterns.iter().any(|re| re.is_match(field)) {
                return true;
            }
        }

        false
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
    ///     redactor.redact_field("password", "s3cr3t"),
    ///     "******"
    /// );
    ///
    /// assert_eq!(
    ///     redactor.redact_field("username", "anna"),
    ///     "anna"
    /// );
    /// ```
    pub fn redact_field(&self, field: &str, value: &str) -> String {
        if self.should_redact_field(field) {
            return self.mask.clone();
        }

        #[cfg_attr(not(feature = "regex"), allow(unused_mut))]
        let mut result = value.to_string();

        #[cfg(feature = "regex")]
        {
            for (re, replacement) in &self.value_patterns {
                result = re.replace_all(&result, replacement.as_str()).into_owned();
            }
        }

        result
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
    /// assert_eq!(redactor.redact_value("s3cr3t"), "******");
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

        assert_eq!(redactor.redact_field("password", "s3cr3t"), "******");
    }

    #[test]
    fn redact_field_returns_original_for_non_sensitive_field() {
        let redactor = Redactor::builder().field("password").build();

        assert_eq!(redactor.redact_field("username", "anna"), "anna");
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

        assert_eq!(redactor.redact_field("password", "s3cr3t"), "[hidden]");
    }

    #[test]
    fn redact_value_always_returns_mask() {
        let redactor = Redactor::builder().field("password").build();

        assert_eq!(redactor.redact_value("anything"), "******");
    }

    #[test]
    fn field_matching_is_case_sensitive_for_now() {
        let redactor = Redactor::builder().field("password").build();

        assert!(redactor.should_redact_field("password"));
        assert!(!redactor.should_redact_field("PASSWORD"));
        assert!(!redactor.should_redact_field("Password"));
    }
}
