

use proc_macro::{TokenStream};

use darling::{FromMeta};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident, Meta, NestedMeta, Lifetime, Lit};

use crate::attrs::Attrs;


/// Decode derive helper
pub fn derive_decode_impl(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);

    // Extract struct fields
    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Unsupported object type for derivation"),
    };

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
        let attrs = Attrs::parse(f.attrs.iter());

        match (attrs.decode, attrs.length) {    
            (Some(d), _) => parsers.extend(quote!{
                let (#id, n) = #d(&buff[index..])?;
                index += n;
            }),
            (_, None) => parsers.extend(quote!{
                let (#id, n) = <#ty>::decode(&buff[index..])?;
                index += n;
            }),
            (_, Some(l)) => parsers.extend(quote!{
                let n = #l as usize;
                let #id = <#ty>::decode_len(&buff[index..], n)?;
                index += n;
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

    quote! {
        impl <'dec: #(#lts)+*, #(#lts),*> ::encdec::Decode<'dec> for #ident #ty_generics #where_clause {
            type Output = Self;
            type Error = ::encdec::Error;
            
            fn decode(buff: &'dec [u8]) -> Result<(Self::Output, usize), Self::Error> {
                use ::encdec::decode::{Decode, DecodedTagged, DecodePrefixed};

                let mut index = 0;
                
                #parsers

                Ok((#obj, index))
            }
        }
    }.into()
}
