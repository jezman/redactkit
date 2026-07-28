//! Shared helpers for parsing `redactkit` derive attributes.

use syn::{Field, Meta};

/// Returns `true` if the field is marked with `#[redact]`.
///
/// Currently only the bare form is supported:
///
/// ```ignore
/// #[redact]
/// ```
pub(crate) fn is_redacted(field: &Field) -> syn::Result<bool> {
    for attr in &field.attrs {
        if !attr.path().is_ident("redact") {
            continue;
        }

        if !matches!(attr.meta, Meta::Path(_)) {
            return Err(syn::Error::new_spanned(
                attr,
                "currently only bare `#[redact]` is supported",
            ));
        }

        return Ok(true);
    }

    Ok(false)
}
