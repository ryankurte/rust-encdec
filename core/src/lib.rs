//! Binary object encoding / decoding traits and helper macros
//! 
//! This crate provides common [`Encode`] and [`Decode`] 
//! traits for describing binary encode/decode-able objects,
//! as well as derive macros to propagate these and 
//! little-endian implementations for primitive types.
//! 
//! ### Derive
//! ```
//! # use encdec::{Encode, Decode, Error};
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct SomeA {
//!     a: u8,
//!     b: u16,
//!     c: u8,
//! }
//! # let mut buff = [0u8; 16];
//! 
//! let a1 = SomeA{ a: 0x10, b: 0xabcd, c: 0x11 };
//! let n1 = a1.encode(&mut buff[..]).unwrap();
//! 
//! // Encoded data is little endian, ordered by struct field
//! assert_eq!(&buff[..n1], &[0x10, 0xcd, 0xab, 0x11]);
//!
//! let (a2, n2) = SomeA::decode(&buff[..n1]).unwrap();
//! assert_eq!(a1, a2);
//! ```
//! 
//! For detail on derived implementations see the [Encode][encdec_macros::Encode] and [Decode][encdec_macros::Decode] macros.

#![no_std]

pub use encdec_base::{
    EncDec, Error,
    encode::{self, Encode, EncodeExt},
    decode::{self, Decode, DecodeOwned, DecodeExt},
};

pub use encdec_macros::{Encode, Decode, DecodeOwned};

pub mod helpers {
    //! Helpers for testing encode/decode objects as well as specialised encode/decode impls
    pub use encdec_base::helpers::*;
}
