//! Errors returned by redactkit.

use core::fmt;

/// Errors returned by redactkit.
#[derive(Debug)]
pub enum Error {
    /// The provided regex pattern failed to compile.
    #[cfg(feature = "regex")]
    InvalidRegex {
        /// The pattern that failed to compile.
        pattern: String,
        /// The underlying regex error.
        source: regex::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "regex")]
        {
            match self {
                Error::InvalidRegex { pattern, source } => {
                    write!(_f, "invalid regex pattern `{pattern}`: {source}")
                }
            }
        }

        #[cfg(not(feature = "regex"))]
        {
            match *self {}
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        #[cfg(feature = "regex")]
        {
            match self {
                Error::InvalidRegex { source, .. } => Some(source),
            }
        }

        #[cfg(not(feature = "regex"))]
        {
            match *self {}
        }
    }
}
