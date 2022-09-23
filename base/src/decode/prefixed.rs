use core::fmt::Debug;

use crate::Error;
use super::Decode;

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
    <P as Decode<'a>>::Output: num_traits::AsPrimitive<usize>,
    <T as Decode<'a>>::Error: From<Error>,
{
    type Output = <T as Decode<'a>>::Output;
    type Error = <T as Decode<'a>>::Error;

    fn decode_prefixed(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        use num_traits::AsPrimitive;

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
