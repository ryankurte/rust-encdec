//! [`Decode`] trait implementation

use core::{fmt::Debug, marker::PhantomData};

use crate::Error;

/// Decode trait implemented for binary decodable objects
pub trait Decode<'a>: Sized {
    /// Output type (allows attaching lifetime bounds where required)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Decode consumes a slice and returns an object and decoded length.
    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error>;
}

/// Decode trait extensions
pub trait DecodeExt<'a>: Decode<'a> {
    /// Helper to iterate over decodable objects in a _sized_ buffer.
    /// 
    /// Note that objects must be -internally- sized as this is a greedy operation
    fn decode_iter(buff: &'a [u8]) -> DecodeIter<'a, Self::Output> {
        DecodeIter {
            buff,
            index: 0,
            _t: PhantomData,
        }
    }
}

impl <'a, T: Decode<'a>> DecodeExt<'a> for T {}

/// Decode for owned types, avoids lifetime constraints
pub trait DecodeOwned {
    /// Output type
    type Output: Debug;

    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Decode consumes a slice and returns an object and decoded length.
    fn decode(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error>;
}

impl <'a, T: DecodeOwned> Decode<'a> for T {
    type Output = <T as DecodeOwned>::Output;

    type Error = <T as DecodeOwned>::Error;

    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        <T as DecodeOwned>::decode(buff)
    }
}


/// Blanket [`Decode`] impl for slices of encodable types
impl <'a, T, const N: usize> Decode<'a> for [T; N] 
where
    T: Decode<'a>,
    //<T as Decode>::Output<'_>: Default + Debug,
    <T as Decode<'a>>::Error: From<Error> + Debug,
{
    type Output = [<T as Decode<'a>>::Output; N];
    type Error = <T as Decode<'a>>::Error;

    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        
        let mut index = 0;
        
        let d = core::array::try_from_fn(|_i| {
            match T::decode(&buff[index..]) {
                Ok((o, l)) => {
                    index += l;
                    Ok(o)
                },
                Err(e) => Err(e),
            }
        })?;

        Ok((d, index))
    }
}

/// Helper type for parsing lists of decodable objects (with internal length delimiters)
#[derive(Debug)]
pub struct DecodeIter<'a, T> {
    buff: &'a [u8],
    index: usize,
    _t: PhantomData<T>,
}

impl<'a, T> DecodeIter<'a, T>
where
    T: Decode<'a, Output = T>,
    <T as Decode<'a>>::Error: From<Error> + Debug,
{
    /// Create a new [`DecodeIter`] instance over the provided buffer
    pub fn new(buff: &'a [u8]) -> Self {
        Self{ buff, index: 0, _t: PhantomData }
    }
}

/// [`Iterator`] implementation
impl<'a, T> Iterator for DecodeIter<'a, T>
where
    T: Decode<'a, Output = T>,
    <T as Decode<'a>>::Error: From<Error> + Debug,
{
    type Item = Result<T, <T as Decode<'a>>::Error>;

    /// Decode and fetch the next item
    fn next(&mut self) -> Option<Self::Item> {
        // Exit on buffer exhaustion
        if self.index == self.buff.len() {
            return None;
        }

        // Decode the next object
        let (v, n) = match T::decode(&self.buff[self.index..]) {
            Ok((v, n)) => (v, n),
            Err(e) => return Some(Err(e)),
        };

        // Increment the index
        self.index += n;

        Some(Ok(v))
    }
}

/// Clone a [`DecodeIter`] instance, note this resets the underlying iterator
impl<'a, T> Clone for DecodeIter<'a, T>
where
    T: Decode<'a, Output = T>,
    <T as Decode<'a>>::Error: From<Error> + Debug,
{
    fn clone(&self) -> Self {
        Self{ buff: self.buff, index: 0, _t: PhantomData }
    }
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

        match core::str::from_utf8(&buff[..len]) {
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
    <P as Decode<'a>>::Output: num_traits::AsPrimitive<usize>,
    <T as Decode<'a>>::Error: From<Error>,
{
    type Output = <T as Decode<'a>>::Output;
    type Error = <T as Decode<'a>>::Error;

    fn decode_prefixed(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        use num_traits::AsPrimitive;

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
