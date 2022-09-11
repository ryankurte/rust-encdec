
use core::{fmt::Debug};

use crate::Error;

/// Encode trait implemented for binary encodable objects
pub trait Encode: Debug {
    /// Error type returned on parse error
    type Error: Debug;

    /// Calculate expected encoded length for an object
    fn encode_len(&self) -> Result<usize, Self::Error>;

    /// Encode method writes object data to the provided writer
    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error>;

    /// Helper to encode to a fixed size buffer
    fn encode_buff<const N: usize>(&self) -> Result<([u8; N], usize), Self::Error> {
        let mut b = [0u8; N];
        let n = self.encode(&mut b)?;
        Ok((b, n))
    }
}

/// Blanket [`Encode`] impl for slices of encodable types
impl <T, E> Encode for &[T] 
where
    T: Encode<Error=E>,
    E: From<Error> + Debug,
{
    type Error = E;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        let mut index = 0;
        for i in 0..self.len() {
            index += self[i].encode_len()?;
        }
        Ok(index)
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        if buff.len() < self.encode_len()? {
            return Err(Error::BufferOverrun.into());
        }

        let mut index = 0;        
        for i in 0..self.len() {
            index += self[i].encode(&mut buff[index..])?
        }

        Ok(index)
    }

}

/// Blanket [`Encode`] impl for arrays of encodable types
impl <T, const N: usize> Encode for [T; N] 
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
            return Err(Error::BufferOverrun.into());
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
            return Err(Error::BufferOverrun.into());
        }

        buff[..d.len()].copy_from_slice(d);

        Ok(d.len())
    }
}
