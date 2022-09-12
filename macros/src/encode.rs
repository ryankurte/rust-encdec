

use darling::FromMeta;
use proc_macro::{TokenStream};

use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, NestedMeta, Meta, Lit};

/// Encode derive helper
pub fn derive_encode_impl(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);

    // Extract struct fields
    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Unsupported object type for derivation"),
    };

    // Fetch bounds for generics
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Build parser for each field
    let mut encoders = quote!{};
    let mut lengths = quote!{};

    s.fields.iter().enumerate().for_each(|(i, f)| {

        let mut l = None;

        // Parse field attributes
        let attribute_args = f.attrs.iter()
            .filter_map(|v| v.parse_meta().ok() )
            .find(|v| v.path().is_ident("encdec"))
            .map(|v| match v {
                Meta::List(l) => Some(l.nested),
                _ => None,
            })
            .flatten();

        if let Some(args) = attribute_args {
            for a in args.iter() {
                let lit = match &a {
                    NestedMeta::Meta(Meta::NameValue(v)) if v.path.is_ident("length_of") => v.lit.clone(),
                    _ => continue,
                };

                match lit {
                    Lit::Str(v) => l = {
                        let f = v.value();
                        let i = syn::Ident::from_string(&f).unwrap();
                        Some(quote!{ #i })
                    },
                    _ => (),
                }
            }
        }

        let id = match f.ident.clone() {
            Some(id) => quote!{ #id },
            None => {
                let id = syn::Index::from(i);
                quote!{ #id }
            },
        };

        let ty = &f.ty;

        let call_encode = match l {
            None => quote!{ 
                index += self.#id.encode(&mut buff[index..])?;
            },
            Some(v) => quote!{ 
                let n = self.#v.encode_len()?;
                index += (n as #ty).encode(&mut buff[index..])?;
            },
        };

        let call_len = quote!{ index += self.#id.encode_len()?; };

        encoders.extend(call_encode);
        lengths.extend(call_len);
    });

    quote! {
        impl #impl_generics ::encdec::Encode for #ident #ty_generics #where_clause {

            type Error = ::encdec::Error;

            fn encode_len(&self) -> Result<usize, Self::Error> {
                use ::encdec::Encode;

                let mut index = 0;
                
                #lengths

                Ok(index)
            }
            
            fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
                use ::encdec::Encode;

                let mut index = 0;
                
                #encoders

                Ok(index)
            }
        }
    }.into()
}
