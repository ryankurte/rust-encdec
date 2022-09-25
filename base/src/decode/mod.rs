//! [`Decode`] trait implementation

use core::fmt::Debug;

use crate::Error;

mod owned;
pub use owned::DecodeOwned;

mod ext;
pub use ext::{DecodeExt, DecodeIter};

mod tagged;
pub use tagged::DecodedTagged;

mod prefixed;
pub use prefixed::DecodePrefixed;

/// Decode trait implemented for binary decodable objects
pub trait Decode<'a>: Sized {
    /// Output type (allows attaching lifetime bounds where required)
    type Output: Debug;

    /// Error type returned on parse error
    type Error: From<Error> + Debug;

    /// Decode consumes a slice and returns an object and decoded length.
    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error>;
}


/// Blanket [`Decode`] impl for slices of encodable types.
/// 
/// Note this is _greedy_ (ie. will continue until the buffer is exhausted).
#[cfg(feature = "nightly")]
impl <'a, T, const N: usize> Decode<'a> for [T; N] 
where
    T: Decode<'a>,
    //<T as Decode>::Output<'_>: Default + Debug,
    <T as Decode<'a>>::Error: From<Error> + Debug,
{
    type Output = [<T as Decode<'a>>::Output; N];
    type Error = <T as Decode<'a>>::Error;

    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
        
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
