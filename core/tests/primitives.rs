
use rand::random;

use encdec::{helpers::test_encode_decode};

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

#[test]
fn encode_decode_i8() {
    let mut buff = [0u8; 256];
    test_encode_decode::<i8>(&mut buff, random());
}

#[test]
fn encode_decode_i16() {
    let mut buff = [0u8; 256];
    test_encode_decode::<i16>(&mut buff, random());
}

#[test]
fn encode_decode_i32() {
    let mut buff = [0u8; 256];
    test_encode_decode::<i32>(&mut buff, random());
}

#[test]
fn encode_decode_i64() {
    let mut buff = [0u8; 256];
    test_encode_decode::<i64>(&mut buff, random());
}
