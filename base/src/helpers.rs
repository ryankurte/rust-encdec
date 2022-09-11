

use crate::{EncDec};

/// Helper for writing encode_decode tests for encodable objects
pub fn test_encode_decode<'a, T>(buff: &'a mut [u8], v: T)
where
    T: EncDec<'a> + PartialEq,
{
    let encoded_len = v.encode(buff).unwrap();

    let (decoded, decoded_len) = T::decode(&buff[..encoded_len]).unwrap();

    assert!(v == decoded, "value: {:?}, decoded: {:?}", v, decoded);
    assert_eq!(encoded_len, decoded_len);
}
