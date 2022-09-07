

use proc_macro::{TokenStream};

use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data};

/// Encode derive helper
pub fn derive_encode_impl(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, data, generics, .. } = parse_macro_input!(input);

    // Extract struct fields
    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("Unsupported object type for derivation"),
    };

    let _g = generics.params;

    // Build parser for each field
    let mut encoders = quote!{};
    let mut lengths = quote!{};

    s.fields.iter().enumerate().for_each(|(i, f)| {
        let (e, l) = match f.ident.clone() {
            Some(id) => {
                (quote!{
                    index += self.#id.encode(&mut buff[index..])?;
                }, quote!{
                    index += self.#id.encode_len()?;
                })
            },
            None => {
                let id = syn::Index::from(i);
                (quote!{
                    index += self.#id.encode(&mut buff[index..])?;
                }, quote!{
                    index += self.#id.encode_len()?;
                })
            }
        };

        encoders.extend(e);
        lengths.extend(l);

    });

    quote! {
        impl encdec_base::Encode for #ident {

            type Error = encdec_base::Error;

            fn encode_len(&self) -> Result<usize, Self::Error> {
                use encdec_base::Encode;

                let mut index = 0;
                
                #lengths

                Ok(index)
            }
            
            fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
                use encdec_base::Encode;

                let mut index = 0;
                
                #encoders

                Ok(index)
            }
        }
    }.into()
}
