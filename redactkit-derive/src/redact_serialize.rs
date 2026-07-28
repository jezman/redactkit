//! Implementation of `#[derive(RedactSerialize)]`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, parse_macro_input};

/// Expands `#[derive(RedactSerialize)]`.
pub(crate) fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_inner(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_inner(_input: &DeriveInput) -> syn::Result<TokenStream2> {
    Ok(TokenStream2::new())
}
