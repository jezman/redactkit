//! Derive macros for redactkit.

mod attrs;
mod redact_debug;
mod redact_serialize;

use proc_macro::TokenStream;

/// Derives `Debug` for structs, redacting fields marked with `#[redact]`.
///
/// # Example
///
/// ```ignore
/// use redactkit::RedactDebug;
///
/// #[derive(RedactDebug)]
/// struct Config {
///     username: String,
///     #[redact]
///     password: String,
/// }
/// ```
///
/// The generated `Debug` implementation will print:
///
/// ```text
/// Config { username: "anna", password: "******" }
/// ```
#[proc_macro_derive(RedactDebug, attributes(redact))]
pub fn derive_redact_debug(input: TokenStream) -> TokenStream {
    redact_debug::expand(input)
}

/// Derives `serde::Serialize` for structs, redacting fields marked with `#[redact]`.
///
/// This macro is intended to be used through `redactkit::RedactSerialize`
/// with the `serde` feature enabled.
///
/// Redacted fields are serialized as the mask string `"******"`.
/// The original values in memory are not modified.
#[proc_macro_derive(RedactSerialize, attributes(redact))]
pub fn derive_redact_serialize(input: TokenStream) -> TokenStream {
    redact_serialize::expand(input)
}
