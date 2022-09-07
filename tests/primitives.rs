
use rand::random;

use encdec::{Encode, Decode, helpers::test_encode_decode};

#[test]
fn encode_decode_u8() {
    test_encode_decode::<u8>(random());
}

#[test]
fn encode_decode_u16() {
    test_encode_decode::<u16>(random());
}

#[test]
fn encode_decode_u32() {
    test_encode_decode::<u32>(random());
}

#[test]
fn encode_decode_u64() {
    test_encode_decode::<u64>(random());
}
