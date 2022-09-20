

use proc_macro::{TokenStream};

use darling::{FromMeta};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident, Meta, NestedMeta, Lifetime, Lit};

use crate::attrs::{FieldAttrs, StructAttrs};


/// Decode derive helper
pub fn derive_decode_impl(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, data, generics, attrs, .. } = parse_macro_input!(input);

    // Extract struct fields
    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Unsupported object type for derivation"),
    };

    // Parse struct attributes
    let struct_attrs = StructAttrs::parse(attrs.iter());


    // Build parser for each field
    let mut parsers = quote!{};
    let mut fields = quote!{};

    // Fetch bounds for generics
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    s.fields.iter().enumerate().for_each(|(i, f)| {
        let ty = &f.ty;

        let id = match f.ident.clone() {
            Some(id) => id,
            None => Ident::new(&format!("_{}", i), ident.span()),
        };

        // Parse field attributes
        let attrs = FieldAttrs::parse(f.attrs.iter());

        match (&attrs.with, &attrs.decode, &attrs.length) {
            (Some(m), _, _) => parsers.extend(quote!{
                let (#id, n) = #m::dec(&buff[_index..])?;
                _index += n;
            }),
            (_, Some(d), _) => parsers.extend(quote!{
                let (#id, n) = #d(&buff[_index..])?;
                _index += n;
            }),
            (_, _, None) => parsers.extend(quote!{
                let (#id, n) = <#ty>::decode(&buff[_index..])?;
                _index += n;
            }),
            (_, _, Some(l)) => parsers.extend(quote!{
                let n = #l as usize;
                let #id = <#ty>::decode_len(&buff[_index..], n)?;
                _index += n;
            }),
        }

        fields.extend(quote!{ #id, })
    });

    let obj = match s.fields {
        Fields::Named(_) => quote!(Self{#fields}),
        Fields::Unnamed(_) => quote!(Self(#fields)),
        Fields::Unit => quote!(Self{#fields}),
    };

    let lts: Vec<_> = generics.lifetimes()
        .map(|v| Lifetime::from(v.lifetime.clone()) )
        .collect();

    // Override error return type if specified
    let err = match struct_attrs.error {
        Some(e) => quote!(#e),
        None => quote!(::encdec::Error),
    };

    quote! {
        impl <'dec: #(#lts)+*, #(#lts),*> ::encdec::Decode<'dec> for #ident #ty_generics #where_clause {
            type Output = Self;
            type Error = #err;
            
            fn decode(buff: &'dec [u8]) -> Result<(Self::Output, usize), Self::Error> {
                use ::encdec::decode::{Decode, DecodedTagged, DecodePrefixed};

                let mut _index = 0;
                
                #parsers

                Ok((#obj, _index))
            }
        }
    }.into()
}
