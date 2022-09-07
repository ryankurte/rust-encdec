
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
    test_encode_decode(Basic{ a: random(), b: random(), c: random(), d: random() });
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
    test_encode_decode(Arrays{ a: [random(), random(), random()] });
}
