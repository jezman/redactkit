mod redact_serialize;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Meta, parse_macro_input};

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
    let input = parse_macro_input!(input as DeriveInput);

    expand_redact_debug(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_redact_debug(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let name_str = name.to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Struct(DataStruct {
        fields: Fields::Named(named_fields),
        ..
    }) = &input.data
    else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "RedactDebug currently supports only structs with named fields",
        ));
    };

    let mut field_outputs = Vec::new();

    for field in &named_fields.named {
        let Some(field_ident) = &field.ident else {
            continue;
        };

        let field_name = field_ident.to_string();

        if is_redacted(field)? {
            field_outputs.push(quote! {
                .field(#field_name, &"******")
            });
        } else {
            field_outputs.push(quote! {
                .field(#field_name, &self.#field_ident)
            });
        }
    }

    Ok(quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#name_str)
                    #(#field_outputs)*
                    .finish()
            }
        }
    })
}

fn is_redacted(field: &syn::Field) -> syn::Result<bool> {
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

#[proc_macro_derive(RedactSerialize, attributes(redact))]
pub fn derive_redact_serialize(input: TokenStream) -> TokenStream {
    redact_serialize::expand(input)
}
