
use core::fmt::Debug;

use crate::Error;
use super::Decode;

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

        match core::str::from_utf8(&buff[..len]) {
            Ok(v) => Ok(v),
            Err(_e) => Err(Error::Utf8Error),
        }
    }
}
