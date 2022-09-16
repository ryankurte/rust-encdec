//! Macros for deriving [`Decode`] and [`Encode`]
//! 

extern crate proc_macro;
use proc_macro::{TokenStream};


mod encode;
mod decode;


/// `#[derive(Encode)]` support.
/// 
/// generates an [`Encode`][encdec_base::Encode] implementation equivalent to calling `.encode()` on each field in order.
/// 
/// for example:
/// ```
/// # use encdec_base::Encode;
/// #[derive(Debug, PartialEq)]
/// struct Something {
///     a: u8,
///     b: u16,
///     c: [u8; 3],
/// }
/// 
/// // `#[derive(Decode)]` equivalent implementation
/// impl Encode for Something {
///   type Error = ();
/// 
///   fn encode_len(&self) -> Result<usize, Self::Error> {
///     Ok(1 + 2 + 3)
///   }
/// 
///   fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
///     let mut index = 0;
///     buff[index] = self.a;
///     index += 1;
/// 
///     buff[1] = self.b as u8;
///     buff[2] = (self.b >> 8) as u8;
///     index += 2;
/// 
///     buff[3..][..3].copy_from_slice(&self.c);
///     index += 3;
///     
///     Ok(index)
///   }
/// }
/// ```
#[proc_macro_derive(Encode, attributes(encdec))]
pub fn derive_encode_impl(input: TokenStream) -> TokenStream {
    encode::derive_encode_impl(input)
}

/// `#[derive(Decode)]` support.
/// 
/// generates a [`Decode`][encdec_base::Decode] implementation equivalent to calling `.decode()` on each field in order.
/// 
/// for example:
/// ```
/// # use encdec_base::Decode;
/// #[derive(Debug, PartialEq)]
/// struct Something {
///     a: u8,
///     b: u16,
///     c: [u8; 3],
/// }
/// 
/// // `#[derive(Decode)]` equivalent implementation
/// impl <'a> Decode<'a> for Something {
///   type Output = Something;
///   type Error = ();
/// 
///   fn decode(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
///     let mut index = 0;
/// 
///     let a = buff[0];
///     index += 1;
/// 
///     let b = buff[1] as u16 | (buff[2] as u16) << 8;
///     index += 2;
/// 
///     let mut c = [0u8; 3];
///     c.copy_from_slice(&buff[3..][..3]);
///     index += 3;
/// 
///     Ok((Self{a, b, c}, index))
///   }
/// }
/// ```
#[proc_macro_derive(Decode, attributes(encdec))]
pub fn derive_decode_impl(input: TokenStream) -> TokenStream {
    decode::derive_decode_impl(input)
}
