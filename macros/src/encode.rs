//! `#[derive(Encode)`] macro implementation

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, TypeParamBound};

use crate::attrs::{FieldAttrs, StructAttrs};

/// Encode derive helper
pub fn derive_encode_impl(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    // Extract struct fields
    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Unsupported object type for derivation"),
    };

    // Parse struct attributes
    let struct_attrs = StructAttrs::parse(attrs.iter());

    // Fetch bounds for generics
    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    // Build parser for each field
    let mut encoders = quote! {};
    let mut lengths = quote! {};

    s.fields.iter().enumerate().for_each(|(i, f)| {
        // Parse field attributes
        let attrs = FieldAttrs::parse(f.attrs.iter());

        // Generate field identifier
        let id = match f.ident.clone() {
            Some(id) => quote! { #id },
            None => {
                let id = syn::Index::from(i);
                quote! { #id }
            }
        };

        let ty = &f.ty;

        let call_encode = match (&attrs.with, &attrs.encode, &attrs.length_of) {
            // Block / module override
            (Some(m), _, _) => quote! {
                _index += #m::enc(&self.#id, &mut buff[_index..])?;
            },
            // Encode method override
            (_, Some(e), _) => quote! {
                _index += #e(&self.#id, &mut buff[_index..])?;
            },
            // Normal fields using normal encode
            (_, _, None) => quote! {
                _index += self.#id.encode(&mut buff[_index..])?;
            },
            // `length_of` types filled using length of target field
            (_, _, Some(v)) => quote! {
                let n = self.#v.encode_len()?;
                _index += (n as #ty).encode(&mut buff[_index..])?;
            },
        };

        let call_len = match (&attrs.with, &attrs.encode_len) {
            // Block / module override
            (Some(m), _) => quote! { _index += #m::enc_len(&self.#id)?; },
            // Encode length override
            (_, Some(l)) => quote! { _index += #l(&self.#id)?; },
            // Default encode length method
            (_, _) => quote! { _index += self.#id.encode_len()?; },
        };

        encoders.extend(call_encode);
        lengths.extend(call_len);
    });

    // Override error return type if specified
    let err = match struct_attrs.error {
        Some(e) => quote!(#e),
        None => quote!(::encdec::Error),
    };

    // Setup where bounds on generic types

    // Extract existing predicates
    let mut where_bounds = match &generics.where_clause {
        Some(v) => v.predicates.iter().map(|v| quote!(#v)).collect(),
        _ => vec![],
    };

    // Add error bounds for Encode types
    for g in generics.type_params() {
        // Look for types with Decode bounds
        let a = g.bounds.iter().find_map(|v| match v {
            TypeParamBound::Trait(t) if t.path.is_ident("Encode") => Some(t),
            _ => return None,
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

    quote! {
        impl #impl_generics ::encdec::Encode for #ident #ty_generics #where_clause {

            type Error = #err;

            fn encode_len(&self) -> Result<usize, Self::Error> {
                use ::encdec::Encode;

                let mut _index = 0;

                #lengths

                Ok(_index)
            }

            fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
                use ::encdec::Encode;

                let mut _index = 0;

                #encoders

                Ok(_index)
            }
        }
    }
    .into()
}
