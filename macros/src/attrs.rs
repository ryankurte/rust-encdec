use proc_macro2::TokenStream;

use darling::{FromMeta};
use quote::{quote};
use syn::{Attribute, Meta, NestedMeta, Lit};


#[derive(Clone, Debug)]
pub struct Attrs {
    /// Reference to length for decoding
    pub length: Option<TokenStream>,

    /// Length descriptor computed when encoding
    pub length_of: Option<TokenStream>,

    /// Override encode method
    pub encode: Option<TokenStream>,

    /// Override encode length method
    pub encode_len: Option<TokenStream>,

    /// Override decode method
    pub decode: Option<TokenStream>,
}

impl Attrs {
    /// Parse [`Attrs`] object from field attributes
    pub fn parse<'a>(attrs: impl Iterator<Item=&'a Attribute>) -> Self {
        // Filter for `encdec` attribute group
        let attribute_args = attrs
            .filter_map(|v| v.parse_meta().ok() )
            .find(|v| v.path().is_ident("encdec"))
            .map(|v| match v {
                Meta::List(l) => Some(l.nested),
                _ => None,
            })
            .flatten();
        
        // Parse encdec attributes
        match attribute_args {
            Some(a) => Attrs::from(a.iter()),
            None => Attrs::default(),
        }
    }
}

impl Default for Attrs {
    fn default() -> Self {
        Self { 
            length: None,
            length_of: None,
            encode: None,
            encode_len: None,
            decode: None,
        }
    }
}

/// Create [`Attrs`] object from [`NestedMeta`] fields
impl <'a, T: Iterator<Item=&'a NestedMeta>> From<T> for Attrs {
    fn from(args: T) -> Self {
        let mut s = Self::default();

        // Iterate through field arguments
        for a in args {
            // Filter NameValue attributes
            let v = match a {
                NestedMeta::Meta(Meta::NameValue(v)) => v,
                _ => continue,
            };

            // Process literal from value
            let l = match &v.lit {
                Lit::Int(v) => Some(quote!{ #v }),
                Lit::Str(v) => {
                    let f = v.value();
                    let i = syn::Ident::from_string(&f).unwrap();
                    Some(quote!{ #i })
                },
                _ => None,
            };

            let l = match l {
                Some(l) => l,
                None => continue,
            };

            // Match keys to set attribute values

            // Lengths for tagged values
            if v.path.is_ident("length") {
                s.length = Some(l.into());
            } else if v.path.is_ident("length_of") {
                s.length_of = Some(l.into());

            // Encode / decode function overrides
            } else if v.path.is_ident("enc") {
                s.encode = Some(l.into());
            } else if v.path.is_ident("enc_len") {
                s.encode_len = Some(l.into());
            } else if v.path.is_ident("dec") {
                s.decode = Some(l.into());
            }
        }

        println!("Attrs: {:?}", s);

        // Return attribute object
        s
    }
}