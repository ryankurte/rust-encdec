

use core::fmt::Debug;

use crate::Error;

/// Decode trait implemented for binary decodable objects
pub trait Decode: Sized {
    /// Error type returned on parse error
    type Error: Debug;

    /// Parse method consumes a slice and returns an object and the remaining slice.
    fn decode<'a>(buff: &'a[u8]) -> Result<(Self, usize), Self::Error>;
}


/// Blanket [`Decode`] impl for slices of encodable types
impl <T, E, const N: usize> Decode for [T; N] 
where
    T: Decode<Error=E> + Default + Debug,
    E: From<Error> + Debug,
{
    type Error = E;

    fn decode(buff: &[u8]) -> Result<(Self, usize), Self::Error> {
        
        let mut index = 0;
        let mut d = core::array::from_fn(|_i| T::default() );
        
        for i in 0..N {
            let (o, l) = T::decode(&buff[index..])?;

            d[i] = o;
            index += l;
        }

        Ok((d, index))
    }
}
