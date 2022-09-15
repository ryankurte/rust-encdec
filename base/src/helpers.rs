//! encdec test and encoding/decoding helpers
//! 

use core::{
    fmt::Debug,
    str::from_utf8,
};

use num_traits::AsPrimitive;

use crate::{EncDec, Decode, Error};

/// Helper for writing encode_decode tests for encodable objects
pub fn test_encode_decode<'a, T>(buff: &'a mut [u8], v: T)
where
    T: EncDec<'a> + PartialEq,
{
    let encoded_len = v.encode(buff).unwrap();
    assert_eq!(encoded_len, v.encode_len().unwrap(), "actual and expected encode_len differ");

    let (decoded, decoded_len) = T::decode(&buff[..encoded_len]).unwrap();

    assert!(v == decoded, "value: {:?}, decoded: {:?}", v, decoded);
    assert_eq!(encoded_len, decoded_len);
}


/// Decode for fields with tagged lengths
/// (length _must_ be specified via `#[encdec(length=...)]` macro)
pub trait DecodedTagged<'a> {
    /// Output type (required for lifetime bounds)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Decode consumes a slice and explicit length and returns an object
    fn decode_len(buff: &'a [u8], len: usize) -> Result<Self::Output, Self::Error>;
}

/// [`DecodedTagged`] impl for byte arrays
/// (requires `#[encdec(length=...)]` length delimiter)
impl <'a>DecodedTagged<'a> for &[u8] {
    type Output = &'a [u8];
    type Error = Error;

    fn decode_len(buff: &'a [u8], len: usize) -> Result<Self::Output, Self::Error> {
        if buff.len() < len {
            return Err(Error::BufferOverrun);
        }

        Ok(&buff[..len])
    }
}

/// [`DecodedTagged`] impl for string slices (`&str`)
/// (requires `#[encdec(length=...)]` length delimiter)
impl <'a>DecodedTagged<'a> for &str {
    type Output = &'a str;
    type Error = Error;

    fn decode_len(buff: &'a [u8], len: usize) -> Result<Self::Output, Self::Error> {
        if buff.len() < len {
            return Err(Error::BufferOverrun);
        }

        match from_utf8(&buff[..len]) {
            Ok(v) => Ok(v),
            Err(_e) => Err(Error::Utf8Error),
        }
    }
}

/// Decode for fields with prefixed lengths
pub trait DecodePrefixed<'a, P: Decode<'a>> {
    /// Output type (required for lifetime bounds)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Decode consumes a slice and returns an object
    fn decode_prefixed(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error>;
}

impl <'a, T, P> DecodePrefixed<'a, P> for T 
where
    T: Decode<'a>,
    P: Decode<'a, Error=Error>,
    <P as Decode<'a>>::Output: AsPrimitive<usize>,
    <T as Decode<'a>>::Error: From<Error>,
{
    type Output = <T as Decode<'a>>::Output;
    type Error = <T as Decode<'a>>::Error;

    fn decode_prefixed(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        let mut index = 0;

        // First, decode prefix
        let (len, n) = P::decode(&buff)?;
        index += n;

        // Then, decode the body using this length
        let (b, n) = T::decode(&buff[index..][..len.as_()])?;
        index += n;

        Ok((b, index))
    }
}
