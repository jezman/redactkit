//! Derive macros for redactkit.

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

/// Derives a redacting serializer for structs.
///
/// This is currently a stub.
#[proc_macro_derive(RedactSerialize, attributes(redact))]
pub fn derive_redact_serialize(input: TokenStream) -> TokenStream {
    redact_serialize::expand(input)
}
