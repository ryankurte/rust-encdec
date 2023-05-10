use core::fmt::Debug;

use num_traits::FromPrimitive;

use super::Encode;
use crate::Error;

/// Encode for fields with length prefixes
pub trait EncodePrefixed<P: Encode> {
    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Parse method consumes a slice and returns an object
    fn encode_prefixed(&self, buff: &mut [u8]) -> Result<usize, Self::Error>;
}

impl<'a, T, P> EncodePrefixed<P> for T
where
    T: Encode,
    P: Encode<Error = Error> + FromPrimitive,
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
