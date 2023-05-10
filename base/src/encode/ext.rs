use super::Encode;

/// Extensions to [`Encode`] trait for encodable types
pub trait EncodeExt<'a>: Encode + Sized + 'a {
    /// Helper to encode iterables
    fn encode_iter(
        items: impl Iterator<Item = &'a Self>,
        buff: &mut [u8],
    ) -> Result<usize, Self::Error> {
        let mut index = 0;
        for i in items {
            index += i.encode(&mut buff[index..])?;
        }
        Ok(index)
    }

    /// Helper to encode to a fixed size buffer
    fn encode_buff<const N: usize>(&self) -> Result<([u8; N], usize), Self::Error> {
        let mut b = [0u8; N];
        let n = self.encode(&mut b)?;
        Ok((b, n))
    }

    /// Helper to encode to a vector, uses `encode_len` to set length
    #[cfg(feature = "alloc")]
    fn encode_vec(&self) -> Result<(alloc::vec::Vec<u8>, usize), Self::Error> {
        let n = self.encode_len()?;
        let mut b = alloc::vec![0u8; n];
        let n = self.encode(&mut b)?;
        Ok((b, n))
    }
}

/// Blanket implementation for all [`Encode`] types
impl<'a, T: Encode + 'a> EncodeExt<'a> for T {}
