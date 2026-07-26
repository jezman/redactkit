use crate::redactor::Redactor;

/// Builder for [`Redactor`].
///
/// # Examples
///
/// ```
/// use redactkit::Redactor;
///
/// let redactor = Redactor::builder()
///     .field("password")
///     .field("token")
///     .build();
///
/// assert!(redactor.should_redact_field("password"));
/// assert!(redactor.should_redact_field("token"));
/// ```
#[derive(Debug)]
pub struct RedactorBuilder {
    fields: Vec<String>,
    mask: String,
}

impl RedactorBuilder {
    /// Creates a new builder with default settings.
    ///
    /// Default mask should be `"******"`.
    pub(crate) fn new() -> Self {
        RedactorBuilder {
            fields: Vec::new(),
            mask: "******".to_string(),
        }
    }

    /// Adds a single field name to redact.
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
    /// ```
    pub fn field(mut self, name: impl Into<String>) -> Self {
        self.fields.push(name.into());
        self
    }

    /// Adds multiple field names to redact.
    ///
    /// # Examples
    ///
    /// ```
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .fields(["password", "token"])
    ///     .build();
    ///
    /// assert!(redactor.should_redact_field("password"));
    /// assert!(redactor.should_redact_field("token"));
    /// ```
    pub fn fields<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.fields.extend(names.into_iter().map(Into::into));
        self
    }

    /// Sets a custom mask.
    ///
    /// # Examples
    ///
    /// ```
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .field("password")
    ///     .mask("[hidden]")
    ///     .build();
    ///
    /// assert_eq!(
    ///     redactor.redact_field("password", "hunter2"),
    ///     "[hidden]"
    /// );
    /// ```
    pub fn mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = mask.into();
        self
    }

    /// Builds the [`Redactor`].
    pub fn build(self) -> Redactor {
        Redactor::from_parts(self.fields, self.mask)
    }
}
