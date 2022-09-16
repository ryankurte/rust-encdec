//! [`Decode`] trait implementation

use core::{fmt::Debug, marker::PhantomData};

use crate::Error;

/// Decode trait implemented for binary decodable objects
pub trait Decode<'a>: Sized {
    /// Output type (allows attaching lifetime bounds where required)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Decode consumes a slice and returns an object and decoded length.
    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error>;

    /// Helper to iterate over decodable objects in a _sized_ buffer.
    /// 
    /// Note that objects must be -internally- sized as this is a greedy operation
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
