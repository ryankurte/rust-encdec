//! Macros for deriving [`encdec::Decode`] and [`encdec::Encode`]
//! 

extern crate proc_macro;
use proc_macro::{TokenStream};


mod encode;
mod decode;


/// [`Encode`] derive helper
#[proc_macro_derive(Encode, attributes(encdec))]
pub fn derive_encode_impl(input: TokenStream) -> TokenStream {
    encode::derive_encode_impl(input)
}

/// [`Decode`] derive helper
#[proc_macro_derive(Decode, attributes(encdec))]
pub fn derive_decode_impl(input: TokenStream) -> TokenStream {
    decode::derive_decode_impl(input)
}

#[derive(Default, darling::FromMeta)]
struct Params {
    pub length: Option<syn::Ident>,
}
