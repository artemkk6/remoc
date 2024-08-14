//! Utility functions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Ident};

/// Converts the identifier to pascal case.
pub fn to_pascal_case(ident: &Ident) -> Ident {
    let s = ident.to_string();
    let mut capital = true;
    let mut out = String::new();
    for c in s.chars() {
        if c == '_' {
            capital = true;
        } else if capital {
            out.push(c.to_uppercase().next().unwrap());
            capital = false;
        } else {
            out.push(c);
        }
    }
    format_ident!("{}", out)
}

/// TokenStream for list of attributes.
pub fn attribute_tokens(attrs: &[Attribute]) -> TokenStream {
    let mut tokens = quote! {};
    for attr in attrs {
        attr.to_tokens(&mut tokens);
    }
    tokens
}


pub fn use_async_trait_tokens(send: bool, is_trait: bool) -> TokenStream {
    #[cfg(feature="use-async-trait")]
    return if send {
        let _ = is_trait;
        quote! { #[::remoc::rtc::async_trait] }
    } else {
        quote! { #[::remoc::rtc::async_trait(?Send)] }
    };

    #[cfg(feature="use-trait-variant")]
    return if !is_trait {
        quote! {}
    } else if send {
        quote! { #[::remoc::rtc::async_trait(Send)] }
    } else {
        quote! { #[::remoc::rtc::async_trait] }
    };

    #[cfg(not(any(feature="use-async-trait", feature="use-trait-variant")))]
    return quote! {};
}