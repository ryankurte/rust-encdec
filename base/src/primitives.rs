//! Encode/Decode implementations for primitive types
//! 

use byteorder::{LittleEndian as LE, ByteOrder};

use crate::{Encode, DecodeOwned, Error};

/// Helper trait to implement encode/decode on fixed size types
trait FixedEncDec: Sized {
    const N: usize;

    fn e(&self, buff: &mut [u8]) -> ();
    fn d(buff: &[u8]) -> Self;
}

/// Helper macro for implementing primitive encode / decode
macro_rules! impl_encdec {
    ($t:ty, $n:literal, $d:expr, $e:expr) => {
        impl DecodeOwned for $t {
            type Output = $t;
            type Error = Error;

            #[inline]
            fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
                if buff.len() < $n {
                    return Err(Error::Length);
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
            fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
                if buff.len() < $n {
                    return Err(Error::Length);
                }

                $e(&mut buff[..$n], *self);

                Ok($n)
            }
        }
    };
}

impl_encdec!(u8,  1, get_u8, put_u8);
impl_encdec!(i8,  1, get_i8, put_i8);
impl_encdec!(u16, 2, LE::read_u16, LE::write_u16);
impl_encdec!(i16, 2, LE::read_i16, LE::write_i16);
impl_encdec!(u32, 4, LE::read_u32, LE::write_u32);
impl_encdec!(i32, 4, LE::read_i32, LE::write_i32);
impl_encdec!(u64, 8, LE::read_u64, LE::write_u64);
impl_encdec!(i64, 8, LE::read_i64, LE::write_i64);


#[inline]
fn get_u8(buff: &[u8]) -> u8 {
    buff[0]
}

#[inline]
fn get_i8(buff: &[u8]) -> i8 {
    buff[0] as i8
}

#[inline]
fn put_u8(buff: &mut [u8], val: u8) {
    buff[0] = val;
}

#[inline]
fn put_i8(buff: &mut [u8], val: i8) {
    buff[0] = val as u8;
}
