
use core::{fmt::Debug};

use num_traits::FromPrimitive;

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

/// Encode for fields with prefixed lengths
pub trait EncodePrefixed<P: Encode> {
    /// Error type returned on parse error
    type Error: Debug;

    /// Parse method consumes a slice and returns an object
    fn encode_prefixed(&self, buff: &mut [u8]) -> Result<usize, Self::Error>;
}

impl <'a, T, P> EncodePrefixed<P> for T 
where
    T: Encode,
    P: Encode<Error=Error> + FromPrimitive,
    <T as Encode>::Error: From<Error>,
{
    type Error = <T as Encode>::Error;

    fn encode_prefixed(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        let mut index = 0;

        // Compute encoded length and write prefix
        let len = P::from_usize(self.encode_len()?).unwrap();
        index += len.encode(buff)?;

        // Encode object
        index += self.encode(&mut buff[index..])?;

        Ok(index)
    }
}
