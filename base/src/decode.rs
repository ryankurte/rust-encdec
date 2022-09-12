

use core::{fmt::Debug, str::from_utf8, marker::PhantomData};

use num_traits::AsPrimitive;


use crate::Error;

/// Decode trait implemented for binary decodable objects
pub trait Decode<'a>: Sized {
    /// Output type (required for lifetime bounds)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Decode consumes a slice and returns an object and decoded length
    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error>;

    /// Helper to iterate over decodable objects in a _sized_ buffer
    fn decode_iter(buff: &'a [u8]) -> DecodeIter<'a, Self::Output, Self::Error> {
        DecodeIter {
            buff,
            index: 0,
            _t: PhantomData,
            _e: PhantomData,
        }
    }
}

/// Blanket [`Decode`] impl for slices of encodable types
impl <'a, T, E, const N: usize> Decode<'a> for [T; N] 
where
    T: Decode<'a, Error=E>,
    //<T as Decode>::Output<'_>: Default + Debug,
    E: From<Error> + Debug,
{
    type Output = [<T as Decode<'a>>::Output; N];
    type Error = E;

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
pub struct DecodeIter<'a, T, E> {
    buff: &'a [u8],
    index: usize,
    _t: PhantomData<T>,
    _e: PhantomData<E>,
}

impl<'a, T, E> DecodeIter<'a, T, E>
where
    T: Decode<'a, Output = T, Error = E>,
{
    /// Create a new [`DecodeIter`] instance over the provided buffer
    pub fn new(buff: &'a [u8]) -> Self {
        Self{ buff, index: 0, _t: PhantomData, _e: PhantomData }
    }
}

/// [`Iterator`] implementation
impl<'a, T, E> Iterator for DecodeIter<'a, T, E>
where
    T: Decode<'a, Output = T, Error = E>,
{
    type Item = Result<T, E>;

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
impl<'a, T, E> Clone for DecodeIter<'a, T, E>
where
    T: Decode<'a, Output = T, Error = E>,
{
    fn clone(&self) -> Self {
        Self{ buff: self.buff, index: 0, _t: PhantomData, _e: PhantomData }
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
