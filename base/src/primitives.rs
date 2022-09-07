//! Encode/Decode implementations for primitive types

use core::fmt::Debug;

use byteorder::{LittleEndian, ByteOrder};

use crate::{Encode, Decode, Error};

/// Helper trait to implement encode/decode on fixed size types
trait FixedEncDec: Sized {
    const N: usize;

    fn e(&self, buff: &mut [u8]) -> ();
    fn d(buff: &[u8]) -> Self;
}

impl <T: FixedEncDec + Debug + Sized> Decode for T {
    type Error = Error;

    fn decode<'a>(buff: &'a[u8]) -> Result<(Self, usize), Self::Error> {
        if buff.len() < T::N {
            return Err(Error::BufferOverrun);
        }

        let d = T::d(&buff[..T::N]);

        Ok((d, T::N))
    }
}

impl <T: FixedEncDec + Debug + Sized> Encode for T {
    type Error = Error;

    fn encode_len(&self) -> Result<usize, Self::Error> {
        Ok(T::N)
    }

    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
        if buff.len() < T::N {
            return Err(Error::BufferOverrun);
        }

        T::e(self, &mut buff[..T::N]);

        Ok(T::N)
    }
}

impl FixedEncDec for u8 {
    const N: usize = 1;

    fn e(&self, buff: &mut [u8]) -> () { buff[0] = *self; }
    fn d(buff: &[u8]) -> Self { buff[0] }
}

impl FixedEncDec for u16 {
    const N: usize = 2;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_u16(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_u16(&buff[..Self::N]) }
}

impl FixedEncDec for u32 {
    const N: usize = 4;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_u32(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_u32(&buff[..Self::N]) }
}

impl FixedEncDec for u64 {
    const N: usize = 8;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_u64(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_u64(&buff[..Self::N]) }
}

impl FixedEncDec for i8 {
    const N: usize = 1;

    fn e(&self, buff: &mut [u8]) -> () { buff[0] = *self as u8; }
    fn d(buff: &[u8]) -> Self { buff[0] as i8 }
}

impl FixedEncDec for i16 {
    const N: usize = 2;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_i16(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_i16(&buff[..Self::N]) }
}

impl FixedEncDec for i32 {
    const N: usize = 4;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_i32(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_i32(&buff[..Self::N]) }
}

impl FixedEncDec for i64 {
    const N: usize = 8;

    fn e(&self, buff: &mut [u8]) -> () { LittleEndian::write_i64(&mut buff[..Self::N], *self) }
    fn d(buff: &[u8]) -> Self { LittleEndian::read_i64(&buff[..Self::N]) }
}
