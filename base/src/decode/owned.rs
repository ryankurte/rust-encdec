
use core::fmt::Debug;

use crate::Error;
use super::Decode;

/// Decode trait implemented for owned types
/// 
/// This allows eliding lifetime constraints for owned (ie. self-contained, not reference) types and provides a blanket [`Decode`] implementation
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

#[cfg(not(feature = "nightly"))]
impl<T, const N: usize> DecodeOwned for [T; N]
where
    T: DecodeOwned<Output=T> + Debug + Default + Copy,
    <T as DecodeOwned>::Error: From<Error> + Debug,
{
    type Error = <T as DecodeOwned>::Error;
    type Output = [<T as DecodeOwned>::Output; N];

    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
        let mut data: [T; N] = [T::default(); N];

        let mut offset = 0;
        for value in data.iter_mut() {
            let (output, length) = T::decode_owned(&buff[offset..])?;
            offset += length;
            *value = output;
        }

        Ok((data, offset))
    }
}


/// [`DecodeOwned`] for `[T; N]`s containing [`DecodeOwned`] types
#[cfg(feature = "nightly")]
impl <T, const N: usize> DecodeOwned for [T; N] 
where
    T: DecodeOwned<Output=T> + Debug,
    <T as DecodeOwned>::Error: From<Error> + Debug,
{
    type Error = <T as DecodeOwned>::Error;

    type Output = [<T as DecodeOwned>::Output; N];

    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
        let mut index = 0;

        let decoded = core::array::try_from_fn(|_i| {
            match T::decode(&buff[index..]) {
                Ok((o, l)) => {
                    index += l;
                    Ok(o)
                },
                Err(e) => Err(e),
            }
        })?;

        Ok((decoded, index))
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
                return Err(Error::Length.into())
            }

            index += n;
        }

        Ok((v, index))
    }
}
