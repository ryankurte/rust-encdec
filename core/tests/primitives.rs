
#![feature(generic_associated_types)]

use rand::random;

use encdec::{Encode, Decode, helpers::test_encode_decode};

#[test]
fn encode_decode_u8() {
    let mut buff = [0u8; 256];
    test_encode_decode::<u8>(&mut buff, random());
}

#[test]
fn encode_decode_u16() {
    let mut buff = [0u8; 256];
    test_encode_decode::<u16>(&mut buff, random());
}

#[test]
fn encode_decode_u32() {
    let mut buff = [0u8; 256];
    test_encode_decode::<u32>(&mut buff, random());
}

#[test]
fn encode_decode_u64() {
    let mut buff = [0u8; 256];
    test_encode_decode::<u64>(&mut buff, random());
}