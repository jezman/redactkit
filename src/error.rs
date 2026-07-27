use thiserror::Error;

/// Errors returned by redactkit.
#[derive(Debug, Error)]
pub enum Error {
    /// The provided regex pattern failed to compile.
    #[cfg(feature = "regex")]
    #[error("invalid regex pattern `{pattern}`: {source}")]
    InvalidRegex {
        /// The pattern that failed to compile.
        pattern: String,

        /// The underlying regex error.
        #[source]
        source: regex::Error,
    },
}
