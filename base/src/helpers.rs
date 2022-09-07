
use crate::{Encode, Decode};

/// Helper for writing encode_decode tests for encodable objects
pub fn test_encode_decode<T: Encode + Decode + PartialEq>(v: T) {
    let mut buff = [0u8; 256];

    let encoded_len = v.encode(&mut buff).unwrap();

    let (decoded, decoded_len) = T::decode(&buff[..encoded_len]).unwrap();

    assert_eq!(v, decoded);
    assert_eq!(encoded_len, decoded_len);
}
