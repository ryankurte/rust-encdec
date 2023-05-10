//! [`Encode`] trait implementation

use core::fmt::Debug;

use crate::Error;

mod prefixed;
pub use prefixed::EncodePrefixed;

mod ext;
pub use ext::EncodeExt;

/// Encode trait implemented for binary encodable objects
pub trait Encode: Debug {
    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Calculate expected encoded length for an object
    fn encode_len(&self) -> Result<usize, Self::Error>;

    /// Encode method writes object data to the provided writer
    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error>;
}

/// Blanket encode for references to encodable types
impl<T: Encode> Encode for &T {
    type Error = <T as Encode>::Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        <T as Encode>::encode_len(self)
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        <T as Encode>::encode(self, buff)
    }
}

/// Blanket [`Encode`] impl for slices of encodable types
impl<T> Encode for &[T]
where
    T: Encode,
    <T as Encode>::Error: From<Error> + Debug,
{
    type Error = <T as Encode>::Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        let mut index = 0;
        for i in 0..self.len() {
            index += self[i].encode_len()?;
        }
        Ok(index)
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        if buff.len() < self.encode_len()? {
            return Err(Error::Length.into());
        }

        let mut index = 0;
        for i in 0..self.len() {
            index += self[i].encode(&mut buff[index..])?
        }

        Ok(index)
    }
}

/// Blanket [`Encode`] impl for arrays of encodable types
impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
    <T as Encode>::Error: From<Error> + Debug,
{
    type Error = <T as Encode>::Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        let mut index = 0;
        for i in 0..N {
            index += self[i].encode_len()?;
        }
        Ok(index)
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        if buff.len() < self.encode_len()? {
            return Err(Error::Length.into());
        }

        let mut index = 0;
        for i in 0..N {
            index += self[i].encode(&mut buff[index..])?
        }

        Ok(index)
    }
}

/// [`Encode`] implementation for [`str`]
impl Encode for &str {
    type Error = Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        Ok(self.as_bytes().len())
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        let d = self.as_bytes();
        if buff.len() < d.encode_len()? {
            return Err(Error::Length.into());
        }

        buff[..d.len()].copy_from_slice(d);

        Ok(d.len())
    }
}

/// [`Encode`] implementation for std/alloc [`alloc::vec::Vec`] containing encodable types
#[cfg(feature = "alloc")]
impl<T> Encode for alloc::vec::Vec<T>
where
    T: Encode,
    <T as Encode>::Error: From<Error> + Debug,
{
    type Error = <T as Encode>::Error;

    #[inline]
    fn encode_len(&self) -> Result<usize, Self::Error> {
        let b: &[T] = self.as_ref();
        b.encode_len()
    }

    #[inline]
    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        let b: &[T] = self.as_ref();
        b.encode(buff)
    }
}

/// [`Encode`] implementation for heapless [`heapless::Vec`] containing encodable types
#[cfg(feature = "heapless")]
impl<T, const N: usize> Encode for heapless::Vec<T, N>
where
    T: Encode,
    <T as Encode>::Error: From<Error> + Debug,
{
    type Error = <T as Encode>::Error;

    #[inline]
    fn encode_len(&self) -> Result<usize, Self::Error> {
        let b: &[T] = self.as_ref();
        b.encode_len()
    }

    #[inline]
    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        let b: &[T] = self.as_ref();
        b.encode(buff)
    }
}
