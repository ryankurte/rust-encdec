//! encdec test and encoding/decoding helpers
//! 

use core::{
    fmt::Debug,
    str::from_utf8,
};

use num_traits::AsPrimitive;

use crate::{EncDec, Decode, Error};

/// Helper for writing encode_decode tests for encodable objects
pub fn test_encode_decode<'a, T>(buff: &'a mut [u8], v: T)
where
    T: EncDec<'a> + PartialEq,
{
    let encoded_len = v.encode(buff).unwrap();
    assert_eq!(encoded_len, v.encode_len().unwrap(), "actual and expected encode_len differ");

    let (decoded, decoded_len) = T::decode(&buff[..encoded_len]).expect("decode failed");

    assert!(v == decoded, "value: {:?}, decoded: {:?}", v, decoded);
    assert_eq!(encoded_len, decoded_len, "encode and decode length differ");
}

