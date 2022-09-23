
use core::fmt::Debug;

use crate::Error;
use super::Decode;

/// Decode trait for owned types, avoids lifetime constraints
pub trait DecodeOwned {
    /// Output type
    type Output: Debug;

    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Decode consumes a slice and returns an object and decoded length.
    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error>;
}

/// Blanket [`Decode`] impl for [`DecodeOwned`] types
impl <'a, T: DecodeOwned> Decode<'a> for T {
    type Output = <T as DecodeOwned>::Output;

    type Error = <T as DecodeOwned>::Error;

    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        <T as DecodeOwned>::decode_owned(buff)
    }
}

/// [`DecodeOwned`] for [`alloc::vec::Vec`]s containing [`DecodeOwned`] types
#[cfg(feature = "alloc")]
impl <T> DecodeOwned for alloc::vec::Vec<T> 
where
    T: DecodeOwned<Output=T> + Debug,
    <T as DecodeOwned>::Error: From<Error> + Debug,
{
    type Error = <T as DecodeOwned>::Error;

    type Output = alloc::vec::Vec<<T as DecodeOwned>::Output>;

    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
        let mut index = 0;
        let mut v = alloc::vec::Vec::new();

        while index < buff.len() {
            let (d, n) = T::decode(&buff[index..])?;

            v.push(d);
            index += n;
        }

        Ok((v, index))
    }
}

/// [`DecodeOwned`] for [`heapless::Vec`]s containing [`DecodeOwned`] types
#[cfg(feature = "heapless")]
impl <T, const N: usize> DecodeOwned for heapless::Vec<T, N> 
where
    T: DecodeOwned<Output=T> + Debug,
    <T as DecodeOwned>::Error: From<Error> + Debug,
{
    type Error = <T as DecodeOwned>::Error;

    type Output = heapless::Vec<<T as DecodeOwned>::Output, N>;

    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
        let mut index = 0;
        let mut v = heapless::Vec::new();

        while index < buff.len() {
            let (d, n) = T::decode(&buff[index..])?;

            if let Err(_e) = v.push(d) {
                return Err(Error::BufferOverrun.into())
            }

            index += n;
        }

        Ok((v, index))
    }
}
