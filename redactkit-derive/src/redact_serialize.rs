//! Implementation of `#[derive(RedactSerialize)]`.

use crate::attrs::is_redacted;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, ext::IdentExt, parse_macro_input};

/// Expands `#[derive(RedactSerialize)]`.
pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_inner(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_inner(input: &DeriveInput) -> syn::Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "RedactSerialize currently supports only structs without generics",
        ));
    }

    let name = &input.ident;
    let name_str = name.unraw().to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Struct(DataStruct {
        fields: Fields::Named(named_fields),
        ..
    }) = &input.data
    else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "RedactSerialize currently supports only structs with named fields",
        ));
    };

    let field_count = named_fields.named.len();
    let mut field_serializations = Vec::new();

    for field in &named_fields.named {
        let Some(field_ident) = &field.ident else {
            continue;
        };

        let field_name = field_ident.unraw().to_string();

        if is_redacted(field)? {
            field_serializations.push(quote! {
                let _ = &self.#field_ident;
                state.serialize_field(#field_name, "******")?;
            });
        } else {
            field_serializations.push(quote! {
                state.serialize_field(#field_name, &self.#field_ident)?;
            });
        }
    }

    Ok(quote! {
        impl #impl_generics ::redactkit::__private::serde::Serialize for #name #ty_generics #where_clause {
            fn serialize<__S>(&self, serializer: __S) -> ::core::result::Result<__S::Ok, __S::Error>
            where
                __S: ::redactkit::__private::serde::Serializer,
            {
                use ::redactkit::__private::serde::ser::SerializeStruct;

                let mut state = serializer.serialize_struct(#name_str, #field_count)?;
                #(#field_serializations)*
                state.end()
            }
        }
    })
}
