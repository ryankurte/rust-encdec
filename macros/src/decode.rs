//! `#[derive(Decode)`] macro implementation

use proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, Ident, Lifetime, TypeParamBound
};

use crate::attrs::{FieldAttrs, StructAttrs};

/// Decode derive helper
pub fn derive_decode_impl(input: TokenStream, owned: bool) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    // Extract struct fields
    match &data.clone() {
        Data::Struct(s) => derive_decode_impl_struct(owned, ident, data, generics, attrs, s.clone()),
        Data::Enum(e) => derive_decode_impl_enum(ident, data, generics, attrs, e.clone()),
        _ => panic!("Unsupported object type for derivation"),
    }
}

fn derive_decode_impl_enum(ident: Ident, data: Data, generics: Generics, attrs: Vec<Attribute>, s: DataEnum) -> TokenStream {
    // Find repr path
    let repr = match attrs.iter().filter_map(|a| a.parse_meta().ok()).find(|a| a.path().is_ident("repr") ) {
        Some(r) => r,
        None => panic!("Cannot derive encdec::Decode for enums without #[repr(type)]"),
    };

    // Extract repr type (for example, #[repr(u8)] becomes u8)
    let base = match &repr {
        syn::Meta::List(list) => match list.nested.first().unwrap() {
            syn::NestedMeta::Meta(meta) => meta.path().get_ident().clone(),
            syn::NestedMeta::Lit(lit) => panic!("Unexpected lit: {lit:?}"),
        },
        _ => panic!("Unexpected repr: {:?}", repr),
    };

    let mut variants = quote!{};

    // Construct matche arms for each variant using the discriminant
    s.variants.iter().for_each(|v| {
        let i = &v.ident;
        let (_, d) = match &v.discriminant {
            Some(d) => d,
            None => panic!("Cannot impl decode for enum {ident} variant {i} (no discriminant)"),
        };

        variants.extend(quote!{
            #d => #ident::#i,
        });
    });

    //panic!("variants: {:?}", variants.to_string());

    // Build decode implementation
    quote! {
        impl ::encdec::DecodeOwned for #ident {
            type Output = Self;
            type Error = encdec::Error;
            
            fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
                use ::encdec::decode::{DecodeOwned};
                
                let (v, n) = <#base as DecodeOwned>::decode_owned(buff)?;

                let obj = match v {
                    #variants
                    _ => return Err(encdec::Error::Variant),
                };

                Ok((obj, n))
            }
        }
    }.into()  
}

fn derive_decode_impl_struct(owned: bool, ident: Ident, _data: Data, generics: Generics, attrs: Vec<Attribute>, s: DataStruct) -> TokenStream {

    // Parse struct attributes
    let struct_attrs = StructAttrs::parse(attrs.iter());

    // Build parser for each field
    let mut parsers = quote! {};
    let mut fields = quote! {};

    // Fetch bounds for generics
    let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    s.fields.iter().enumerate().for_each(|(i, f)| {
        let ty = &f.ty;

        let id = match f.ident.clone() {
            Some(id) => id,
            None => Ident::new(&format!("_{}", i), ident.span()),
        };

        // Parse field attributes
        let attrs = FieldAttrs::parse(f.attrs.iter());

        match (&attrs.with, &attrs.decode, &attrs.length) {
            (Some(m), _, _) => parsers.extend(quote! {
                let (#id, n) = #m::dec(&buff[_index..])?;
                _index += n;
            }),
            (_, Some(d), _) => parsers.extend(quote! {
                let (#id, n) = #d(&buff[_index..])?;
                _index += n;
            }),
            (_, _, Some(l)) => parsers.extend(quote! {
                let n = #l as usize;
                let #id = <#ty>::decode_len(&buff[_index..], n)?;
                _index += n;
            }),
            (_, _, None) => parsers.extend(quote! {
                let (#id, n) = <#ty>::decode(&buff[_index..])?;
                let #id = #id.into();
                _index += n;
            }),
        }

        fields.extend(quote! { #id, })
    });

    let obj = match s.fields {
        Fields::Named(_) => quote!(Self{#fields}),
        Fields::Unnamed(_) => quote!(Self(#fields)),
        Fields::Unit => quote!(Self{#fields}),
    };

    let lifetimes: Vec<_> = generics
        .lifetimes()
        .map(|v| Lifetime::from(v.lifetime.clone()))
        .collect();

    let generic_types: Vec<_> = generics.type_params().collect();

    let const_params: Vec<_> = generics
        .const_params()
        .map(|v| {
            let mut v = v.clone();
            v.eq_token = None;
            v.default = None;
            v
        })
        .collect();

    // Override error return type if specified
    let err = match struct_attrs.error {
        Some(e) => quote!(#e),
        None => quote!(::encdec::Error),
    };

    // Extract where bounds
    let mut where_bounds = match &generics.where_clause {
        Some(v) => v.predicates.iter().map(|v| quote!(#v)).collect(),
        _ => vec![],
    };

    // Add where bounds for Decode types
    for g in &generic_types {
        // Look for types with Decode bounds
        let a = g.bounds.iter().find_map(|v| {
            // Find trait bounds
            let t = match v {
                TypeParamBound::Trait(t) => t,
                _ => return None,
            };

            // Match decode bounds
            let _s = match t.path.segments.first() {
                Some(v) if v.ident == "Decode" => v,
                Some(v) if v.ident == "DecodeOwned" => v,
                _ => return None,
            };

            Some(v)
        });

        // Skip non-Decode types (probably not possible?)
        let a = match a {
            Some(v) => v,
            None => continue,
        };

        // Fetch type
        let t = &g.ident;

        // Append where clause
        let w = quote!(
            #t: From<<#t as #a>::Output>,
            #err: From<<#t as #a>::Error>,
        );

        where_bounds.push(w);
    }

    // Build where clause
    let mut where_clause = None;
    if where_bounds.len() > 0 {
        where_clause = Some(quote! {
            where
                #(#where_bounds),*
        });
    }

    //panic!("bounds: {}", TokenStream::from(where_clause.unwrap()));

    match owned {
        false => quote! {
            impl <'dec: #(#lifetimes)+*, #(#lifetimes),* #(#generic_types),* #(#const_params),*> ::encdec::Decode<'dec> for #ident #ty_generics #where_clause {
                type Output = Self;
                type Error = #err;
                
                fn decode(buff: &'dec [u8]) -> Result<(Self::Output, usize), Self::Error> {
                    use ::encdec::decode::{Decode, DecodedTagged, DecodePrefixed};

                    let mut _index = 0;
                    
                    #parsers

                    Ok((#obj, _index))
                }
            }
        },
        true => quote! {
            impl <#(#lifetimes),* #(#generic_types),* #(#const_params),*> ::encdec::DecodeOwned for #ident #ty_generics #where_clause {
                type Output = Self;
                type Error = #err;
                
                fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
                    use ::encdec::decode::{DecodeOwned};

                    let mut _index = 0;
                    
                    #parsers

                    Ok((#obj, _index))
                }
            }
        },
    }.into()
}
