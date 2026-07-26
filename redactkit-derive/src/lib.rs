use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, parse_macro_input};

/// Derives `Debug` for structs, redacting fields marked with `#[redact]`.
///
/// This is currently a scaffold: it generates a `Debug` impl that prints
/// only the struct name. Field printing will be added in the next step.
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
        fields: Fields::Named(_),
        ..
    }) = &input.data
    else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "RedactDebug currently supports only structs with named fields",
        ));
    };

    Ok(quote! {
        impl #impl_generics ::core::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(#name_str).finish()
            }
        }
    })
}
