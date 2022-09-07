

use proc_macro::{TokenStream};

use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};

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

    let _g = generics.params;

    s.fields.iter().enumerate().for_each(|(i, f)| {
        let ty = &f.ty;

        let id = match f.ident.clone() {
            Some(id) => id,
            None => Ident::new(&format!("_{}", i), ident.span()),
        };

        parsers.extend(quote!{
            let (#id, n) = <#ty>::decode(&buff[index..])?;
            index += n;
        });

        fields.extend(quote!{ #id, })
    });

    let obj = match s.fields {
        Fields::Named(_) => quote!(Self{#fields}),
        Fields::Unnamed(_) => quote!(Self(#fields)),
        Fields::Unit => quote!(Self{#fields}),
    };

    quote! {
        impl encdec_base::Decode for #ident {
            type Error = encdec_base::Error;
            
            fn decode<'a>(buff: &'a [u8]) -> Result<(Self, usize), Self::Error> {
                use encdec_base::Decode;

                let mut index = 0;
                
                #parsers

                Ok((#obj, index))
            }
        }
    }.into()
}
