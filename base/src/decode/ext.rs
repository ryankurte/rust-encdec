
use core::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::Error;
use super::Decode;

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
