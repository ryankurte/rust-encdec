

use core::fmt::Debug;

use crate::Error;

/// Decode trait implemented for binary decodable objects
pub trait Decode<'a>: Sized {
    /// Output type (required for lifetime bounds)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Parse method consumes a slice and returns an object and the remaining slice.
    fn decode(buff: &'a[u8]) -> Result<(Self::Output, usize), Self::Error>;
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

/// Decode for fields with tagged lengths
pub trait DecodedTagged<'a> {
    /// Output type (required for lifetime bounds)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: Debug;

    /// Parse method consumes a slice and returns an object and the remaining slice.
    fn decode_len(buff: &'a [u8], len: usize) -> Result<Self::Output, Self::Error>;
}

/// Blanket [`Decode`] impl for pointers to slices
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

#[cfg(nyet)]
impl DecodedTagged for &str {
    type Output<'a> = &str;
    type Error = Error;

    fn decode_len(buff: &'a [u8], len: usize) -> Result<Self, Self::Error> {
        todo!("how to pass in length via -derive-?")
    }
}
