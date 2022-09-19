//! Encode/Decode implementations for primitive types
//! 

use byteorder::{LittleEndian as LE, ByteOrder};
use bytes::BufMut;

use crate::{Encode, Decode, Error};

/// Helper trait to implement encode/decode on fixed size types
trait FixedEncDec: Sized {
    const N: usize;

    fn e(&self, buff: &mut [u8]) -> ();
    fn d(buff: &[u8]) -> Self;
}

/// Helper macro for implementing primitive encode / decode
macro_rules! impl_encdec {
    ($t:ty, $n:literal, $d:expr, $e:ident) => {
        impl <'a> Decode<'a> for $t {
            type Output = $t;
            type Error = Error;

            #[inline]
            fn decode(buff: &'a[u8]) -> Result<(Self::Output, usize), Self::Error> {
                if buff.len() < $n {
                    return Err(Error::BufferOverrun);
                }

                let v = $d(&buff[..$n]);

                Ok((v, $n))
            }
        }

        impl Encode for $t {
            type Error = Error;

            #[inline]
            fn encode_len(&self) -> Result<usize, Self::Error> {
                Ok($n)
            }

            #[inline]
            fn encode(&self, mut buff: impl BufMut) -> Result<usize, Self::Error> {
                if buff.remaining_mut() < $n {
                    return Err(Error::BufferOverrun);
                }

                BufMut::$e(&mut buff, *self);

                Ok($n)
            }
        }
    };
}

impl_encdec!(u8,  1, get_u8, put_u8);
impl_encdec!(i8,  1, get_i8, put_i8);
impl_encdec!(u16, 2, LE::read_u16, put_u16_le);
impl_encdec!(i16, 2, LE::read_i16, put_i16_le);
impl_encdec!(u32, 4, LE::read_u32, put_u32_le);
impl_encdec!(i32, 4, LE::read_i32, put_i32_le);
impl_encdec!(u64, 8, LE::read_u64, put_u64_le);
impl_encdec!(i64, 8, LE::read_i64, put_i64_le);


#[inline]
fn get_u8(buff: &[u8]) -> u8 {
    buff[0]
}

#[inline]
fn get_i8(buff: &[u8]) -> i8 {
    buff[0] as i8
}