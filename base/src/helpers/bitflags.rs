use core::fmt::Debug;

use bitflags::{Flag, Flags, Bits};

use crate::{decode::DecodeOwned, encode::Encode, EncDec, Error};

/// Marker trait for auto-implementing encode and decode for bitflags objects
/// 
/// Note: this only works where Flags::Bits is a primitive supported by encdec
pub trait EncDecBitflags {}

impl <T: Flags + Debug + EncDecBitflags> Encode for T where 
    <T as Flags>::Bits: Encode<Error = Error>,
{
    type Error = crate::Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        Ok(size_of::<<T as Flags>::Bits>())
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        // Check buffer length
        if buff.len() < self.encode_len()? {
            return Err(Error::Length);
        }

        // Fetch bits repr
        let b = self.bits();

        // Encode using default encoder
        b.encode(buff)
    }
}

impl <T: Flags + Debug + EncDecBitflags> DecodeOwned for T where 
    <T as Flags>::Bits: DecodeOwned<Output = <T as Flags>::Bits, Error = Error>,
{
    type Error = crate::Error;
    
    type Output = T;
    
    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
        if buff.len() < size_of::<<T as Flags>::Bits>() {
            return Err(Error::Length);
        }

        let (bits, n) = <T as Flags>::Bits::decode_owned(buff)?;

        let f = <T as Flags>::from_bits_retain(bits);

        Ok((f, n))
    }
}

#[cfg(test)]
mod test {
    use bitflags::bitflags;
    
    use crate::helpers::test_encode_decode;

    use super::*;

    bitflags!{
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct TestFlags: u16 {
            const A = 0b0001;
            const B = 0b0010;
        }
    }

    impl EncDecBitflags for TestFlags {}

    #[test]
    fn bitflags_encode_decode() {
        let tests = &[
            TestFlags::A,
            TestFlags::B,
            TestFlags::empty(),
            TestFlags::all(),
        ];


        let mut buff = [0u8; 32];
        for t in tests {
            test_encode_decode(&mut buff, *t);
        }
    }

    #[test]
    fn bitflags_encode_fails() {
        
        let a = TestFlags::A;
        let mut b = [0u8; 1];

        // Check encode buffer length
        a.encode(&mut b).expect_err("buffer too short");
        // Check decode buffer length
        TestFlags::decode_owned(&b).expect_err("buffer too short");
    }
}
