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

pub fn trait_macro_attribute_tokens(send: bool) -> TokenStream {
    #[cfg(feature="macro-async-trait")]
    return if send {
        quote! { #[::remoc::rtc::async_trait] }
    } else {
        quote! { #[::remoc::rtc::async_trait(?Send)] }
    };

    #[cfg(feature="macro-trait-variant")]
    return if send {
        quote! { #[::remoc::rtc::async_trait(Send)] }
    } else {
        quote! { #[::remoc::rtc::async_trait] }
    };

    #[cfg(not(any(feature="macro-async-trait", feature="macro-trait-variant")))]
    return quote! {};
}

pub fn impl_macro_attribute_tokens(send: bool) -> TokenStream {
    #[cfg(feature="macro-async-trait")]
    return if send {
        quote! { #[::remoc::rtc::async_trait] }
    } else {
        quote! { #[::remoc::rtc::async_trait(?Send)] }
    };


    #[cfg(not(feature="macro-async-trait"))]
    return quote! {};
}
