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
    #[cfg(feature = "regex")]
    field_patterns: Vec<regex::Regex>,

    #[cfg(feature = "regex")]
    value_patterns: Vec<(regex::Regex, String)>,
}

impl RedactorBuilder {
    /// Creates a new builder with default settings.
    ///
    /// Default mask should be `"******"`.
    pub(crate) fn new() -> Self {
        RedactorBuilder {
            fields: Vec::new(),
            mask: "******".to_string(),

            #[cfg(feature = "regex")]
            field_patterns: Vec::new(),

            #[cfg(feature = "regex")]
            value_patterns: Vec::new(),
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
    ///     redactor.redact_field("password", "s3cr3t"),
    ///     "[hidden]"
    /// );
    /// ```
    pub fn mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = mask.into();
        self
    }

    /// Builds the [`Redactor`].
    pub fn build(self) -> Redactor {
        Redactor {
            fields: self.fields,
            mask: self.mask,

            #[cfg(feature = "regex")]
            field_patterns: self.field_patterns,

            #[cfg(feature = "regex")]
            value_patterns: self.value_patterns,
        }
    }

    /// Adds a regex pattern matched against field names.
    ///
    /// This method is available only with the `regex` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "regex")]
    /// # {
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .field_pattern("(?i)token|secret")
    ///     .unwrap()
    ///     .build();
    ///
    /// assert!(redactor.should_redact_field("api_token"));
    /// assert!(redactor.should_redact_field("CLIENT_SECRET"));
    /// assert!(!redactor.should_redact_field("username"));
    /// # }
    /// ```
    #[cfg(feature = "regex")]
    pub fn field_pattern(mut self, pattern: &str) -> Result<Self, crate::Error> {
        let re = regex::Regex::new(pattern).map_err(|source| crate::Error::InvalidRegex {
            pattern: pattern.to_string(),
            source,
        })?;

        self.field_patterns.push(re);

        Ok(self)
    }

    /// Adds a regex pattern matched against field values.
    ///
    /// Matching parts of the value will be replaced with `replacement`.
    ///
    /// This method is available only with the `regex` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "regex")]
    /// # {
    /// use redactkit::Redactor;
    ///
    /// let redactor = Redactor::builder()
    ///     .value_pattern(r"\d{4}", "****")
    ///     .unwrap()
    ///     .build();
    ///
    /// assert_eq!(
    ///     redactor.redact_field("note", "card 1234 ok"),
    ///     "card **** ok"
    /// );
    /// # }
    /// ```
    #[cfg(feature = "regex")]
    pub fn value_pattern(mut self, pattern: &str, replacement: &str) -> Result<Self, crate::Error> {
        let re = regex::Regex::new(pattern).map_err(|source| crate::Error::InvalidRegex {
            pattern: pattern.to_string(),
            source,
        })?;

        self.value_patterns.push((re, replacement.to_string()));

        Ok(self)
    }
}
