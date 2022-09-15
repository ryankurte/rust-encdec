//! Simple object encoding / decoding traits and helpers
//! 
//! This is intend to provide common traits for marking objects as binary encode/decode-able,
//! as well as derive macros to propagate these traits through structures without manual effort.
//! 
//! See [`encdec_base::Encode`] and [`encdec_base::Decode`] traits for more information.
//! 
//! 
//! ### Encode
//! ```
//! use encdec::{Encode, Decode};
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct Basic {
//!     a: u8,
//!     b: u16,
//! }
//! 
//! let t = Basic{ a: 0x10, b: 0xabcd };
//! 
//! let mut buff = [0u8; 16];
//! let n = t.encode(&mut buff).expect("encode failed");
//! 
//! assert_eq!(&buff[..n], &[0x10, 0xcd, 0xab]);
//! assert_eq!(n, 3);
//! 
//! ```
//! //! ### Decode
//! ```
//! use encdec::{Encode, Decode};
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct Basic {
//!     a: u8,
//!     b: u16,
//! }
//! 
//! let (t, n) = Basic::decode(&[0x12, 0xcd, 0xab]).expect("decode failed");
//! 
//! assert_eq!(t, Basic{ a: 0x12, b: 0xabcd });
//! assert_eq!(n, 3);
//! 
//! ```

#![feature(generic_associated_types)]

#![no_std]

pub use encdec_base::{
    Encode, Decode, Error,
};

pub use encdec_macros::{Encode, Decode};

pub mod helpers {
    //! Helpers for testing encode/decode objects as well as specialised encode/decode impls
    pub use encdec_base::helpers::*;
}
