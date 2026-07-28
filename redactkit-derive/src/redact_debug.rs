//! Implementation of `#[derive(RedactDebug)]`.

use crate::attrs::is_redacted;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, parse_macro_input};

/// Expands `#[derive(RedactDebug)]`.
pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_inner(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

fn expand_inner(input: DeriveInput) -> syn::Result<TokenStream2> {
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

    let mut redacted_reads = Vec::new();
    let mut field_outputs = Vec::new();

    for field in &named_fields.named {
        let Some(field_ident) = &field.ident else {
            continue;
        };

        let field_name = field_ident.to_string();

        if is_redacted(field)? {
            redacted_reads.push(quote! {
                let _ = &self.#field_ident;
            });

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
                #(#redacted_reads)*

                f.debug_struct(#name_str)
                    #(#field_outputs)*
                    .finish()
            }
        }
    })
}
