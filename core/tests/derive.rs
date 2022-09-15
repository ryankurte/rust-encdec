
#![feature(generic_associated_types)]

use rand::random;

use encdec::{Encode, Decode, helpers::test_encode_decode};


#[derive(Debug, PartialEq, Encode, Decode)]
struct Basic {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
}

#[test]
fn basic_derive() {
    let mut buff = [0u8; 256];

    test_encode_decode(&mut buff, Basic{ a: random(), b: random(), c: random(), d: random() });
}

#[test]
fn basic_layout() {
    let t = Basic{ a: random(), b: random(), c: random(), d: random() };
    let mut buff = [0u8; 256];

    let n = t.encode(&mut buff).unwrap();
    assert_eq!(n, 15);

    assert_eq!(buff[0], t.a);
    assert_eq!(&buff[1..][..2], &t.b.to_le_bytes());
    assert_eq!(&buff[3..][..4], &t.c.to_le_bytes());
    assert_eq!(&buff[7..][..8], &t.d.to_le_bytes());
}


#[derive(Debug, PartialEq, Encode, Decode)]
struct Arrays {
    a: [u8; 3],
}

#[test]
fn array_derive() {
    let mut buff = [0u8; 256];

    test_encode_decode(&mut buff, Arrays{ a: [random(), random(), random()] });
}


/// EXPERIMENTAL References with length descriptors
/// 
/// perhaps better to have a "delimited" mode? support for headers? 
/// just require manual encode/decode impls?
#[derive(Debug, PartialEq, Encode, Decode)]
struct Refs<'a> {
    #[encdec(length_of = "a")]
    l: u8,

    #[encdec(length = "l")]
    a: &'a [u8],
}

#[test]
fn ref_derive() {
    let mut buff = [0u8; 256];

    test_encode_decode(&mut buff, Refs{ l: 3, a: &[random(), random(), random()] });
}

#[test]
fn ref_encode_len() {
    let mut buff = [0u8; 256];
    let t = Refs{ l: 0, a: &[random(), random()] };

    let encoded_len = t.encode(&mut buff).unwrap();
    assert_eq!(encoded_len, 3);

    let (d, decoded_len) = Refs::decode(&buff[..encoded_len]).unwrap();

    assert_eq!(d, Refs{ l: 2, a: t.a });
    assert_eq!(encoded_len, decoded_len);
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tuple(u8, u16);

#[test]
fn tuple_derive() {
    let mut buff = [0u8; 256];

    test_encode_decode(&mut buff, Tuple(random(), random()) );
}