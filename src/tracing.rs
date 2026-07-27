//! Tracing integration for redactkit.
//!
//! The tracing integration redacts formatted output.
//! It does not modify or erase original field values in memory.

use crate::Redactor;
use tracing::field::{Field, Visit};
use tracing_subscriber::field::RecordFields;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format::Writer;

/// A redacting field formatter for `tracing-subscriber`.
///
/// Create one via [`redact_fields`], optionally configure it with the
/// builder-style methods, then pass it to
/// `tracing_subscriber::fmt().fmt_fields(...)`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "tracing")]
/// # {
/// use tracing_subscriber::fmt;
///
/// fmt()
///     .fmt_fields(redactkit::tracing::redact_fields())
///     .init();
/// # }
/// ```
#[derive(Debug)]
pub struct RedactFields {
    redactor: Redactor,
}

/// Creates a new redacting field formatter with default sensitive fields.
///
/// The returned formatter redacts common field names such as `password`,
/// `token`, and `api_key`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "tracing")]
/// # {
/// use tracing_subscriber::fmt;
///
/// fmt()
///     .fmt_fields(redactkit::tracing::redact_fields())
///     .init();
/// # }
/// ```
pub fn redact_fields() -> RedactFields {
    RedactFields {
        redactor: crate::default_redactor(),
    }
}

impl RedactFields {
    /// Adds a field name to redact.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "tracing")]
    /// # {
    /// use redactkit::tracing::redact_fields;
    ///
    /// let fields = redact_fields().field("session_id");
    /// # }
    /// ```
    pub fn field(mut self, name: impl Into<String>) -> Self {
        self.redactor.fields.push(name.into());
        self
    }

    /// Adds multiple field names to redact.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "tracing")]
    /// # {
    /// use redactkit::tracing::redact_fields;
    ///
    /// let fields = redact_fields().fields(["session_id", "cookie"]);
    /// # }
    /// ```
    pub fn fields<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.redactor
            .fields
            .extend(names.into_iter().map(Into::into));
        self
    }

    /// Sets a custom mask.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "tracing")]
    /// # {
    /// use redactkit::tracing::redact_fields;
    ///
    /// let fields = redact_fields().mask("[hidden]");
    /// # }
    /// ```
    pub fn mask(mut self, mask: impl Into<String>) -> Self {
        self.redactor.mask = mask.into();
        self
    }

    /// Adds a regex pattern matched against field names.
    ///
    /// This method is available only with the `regex` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(all(feature = "tracing", feature = "regex"))]
    /// # {
    /// use redactkit::tracing::redact_fields;
    ///
    /// let fields = redact_fields()
    ///     .field_pattern("(?i)token")
    ///     .unwrap();
    /// # }
    /// ```
    #[cfg(feature = "regex")]
    pub fn field_pattern(mut self, pattern: &str) -> Result<Self, crate::Error> {
        self.redactor.field_patterns.push(compile(pattern)?);
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
    /// # #[cfg(all(feature = "tracing", feature = "regex"))]
    /// # {
    /// use redactkit::tracing::redact_fields;
    ///
    /// let fields = redact_fields()
    ///     .value_pattern(r"\d{4}", "****")
    ///     .unwrap();
    /// # }
    /// ```
    #[cfg(feature = "regex")]
    pub fn value_pattern(mut self, pattern: &str, replacement: &str) -> Result<Self, crate::Error> {
        let re = compile(pattern)?;
        self.redactor
            .value_patterns
            .push((re, replacement.to_string()));
        Ok(self)
    }
}

#[cfg(feature = "regex")]
fn compile(pattern: &str) -> Result<regex::Regex, crate::Error> {
    regex::Regex::new(pattern).map_err(|source| crate::Error::InvalidRegex {
        pattern: pattern.to_string(),
        source,
    })
}

/// Visitor that collects event fields into owned strings.
struct FieldVisitor {
    /// `(field_name, raw_value, is_string)`.
    fields: Vec<(String, String, bool)>,
}

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            return;
        }

        self.fields
            .push((field.name().to_string(), format!("{value:?}"), false));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            return;
        }

        self.fields
            .push((field.name().to_string(), value.to_string(), true));
    }
}

impl<'writer> FormatFields<'writer> for RedactFields {
    fn format_fields<R: RecordFields>(
        &self,
        mut writer: Writer<'writer>,
        fields: R,
    ) -> std::fmt::Result {
        let mut visitor = FieldVisitor { fields: Vec::new() };
        fields.record(&mut visitor);

        for (name, raw, is_str) in &visitor.fields {
            let effective = self.redactor.redact_field(name, raw);

            let display = if *is_str {
                format!("{effective:?}")
            } else {
                effective
            };

            write!(writer, " {name}={display}")?;
        }

        Ok(())
    }
}
