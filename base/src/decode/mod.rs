//! [`Decode`] trait implementation

use core::fmt::Debug;

use crate::Error;

mod owned;
pub use owned::{DecodeOwned};

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
