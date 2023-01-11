//! Common traits and helper macros for binary encoding and decoding.
//!
//! This crate provides common [`Encode`] and [`Decode`] traits for describing binary encode/decode-able objects, as well as [`#[derive(Encode)]`][encdec_macros::Encode] and [`#[derive(Decode)]`][encdec_macros::Decode] macros to reduce boilerplate when propagating these and little-endian implementations for primitive types.
//! For more implementation see the module-level documentation.
//!
//! This is intended to provide a reusable base for implementing binary encodable objects to match a particular protocol or specification (and mitigate embedded `pub trait (Encode|Decode) {..}` bankruptcy)
//!
//! If you don't need to control the encoded layout directly you might like to look at [serde](https://crates.io/crates/serde), with [postcard](https://crates.io/crates/postcard) for efficient binary encoding between rust components.
//! If you're creating a new protocol for cross-language use you may wish to consider creating a specification using [protocol buffers](https://developers.google.com/protocol-buffers) and [prost](https://crates.io/crates/prost).
//!

#![no_std]

// Re-export base traits
pub use encdec_base::{EncDec, Error};

// Re-export traits from modules here
pub use crate::decode::{Decode, DecodeExt, DecodeOwned};
pub use crate::encode::{Encode, EncodeExt};

pub mod encode {
    //! [`Encode`] traits and helper macros
    //!
    //! ## Example
    //! ```
    //! # use encdec::{Encode, Decode, Error};
    //! #[derive(Debug, PartialEq)]
    //! struct Something {
    //!     a: u8,
    //!     b: u16,
    //!     c: [u8; 3],
    //! }
    //!
    //! impl Encode for Something {
    //!     type Error = Error;
    //!
    //!     /// Fetch encoded length for an encodable object    
    //!     fn encode_len(&self) -> Result<usize, Self::Error> {
    //!         Ok(1 + 2 + 3)
    //!     }
    //!
    //!    /// Encode object to the provided buffer,
    //!    /// returning the encoded length on success
    //!    fn encode(&self, buff: &mut [u8]) -> Result<usize, Self::Error> {
    //!        let mut index = 0;
    //!        buff[index] = self.a;
    //!        index += 1;
    //!
    //!        buff[1] = self.b as u8;
    //!        buff[2] = (self.b >> 8) as u8;
    //!        index += 2;
    //!
    //!        buff[3..][..3].copy_from_slice(&self.c);
    //!        index += 3;
    //!        
    //!        Ok(index)
    //!    }
    //! }
    //! ```

    pub use encdec_base::encode::*;

    pub use crate::derive::Encode;
}

pub mod decode {
    //! [`Decode`] traits and helper macros
    //!
    //! ## Example
    //! ```
    //! # use encdec::{Encode, DecodeOwned, Error};
    //! #[derive(Debug, PartialEq)]
    //! struct SomethingOwned {
    //!     a: u8,
    //!     b: u16,
    //!     c: [u8; 3],
    //! }
    //!
    //! /// [`DecodeOwned`] implementation for self-contained types.
    //! impl DecodeOwned for SomethingOwned {
    //!     type Output = Self;
    //!     type Error = Error;
    //!
    //!    /// Decode object from provided buffer, returning object and decoded
    //!    /// length on success
    //!    fn decode_owned(buff: &[u8]) -> Result<(Self::Output, usize), Self::Error> {
    //!         let mut index = 0;
    //!
    //!         let a = buff[0];
    //!         index += 1;
    //!
    //!         let b = buff[1] as u16 | (buff[2] as u16) << 8;
    //!         index += 2;
    //!
    //!         let mut c = [0u8; 3];
    //!         c.copy_from_slice(&buff[3..][..3]);
    //!         index += 3;
    //!
    //!         Ok((Self{a, b, c}, index))
    //!    }
    //! }
    //! ```
    //!
    //! ```
    //! # use encdec::{Encode, Decode, Error};
    //! #[derive(Debug, PartialEq)]
    //! struct SomethingBorrowed<'a> {
    //!     a: u8,
    //!     l: u16,
    //!     c: &'a [u8],
    //! }
    //!
    //! /// Base [`Decode`] implementation, lifetime support for views
    //! /// into borrowed buffers. If you don't need this, see [`DecodeOwned`].
    //! impl <'a>Decode<'a> for SomethingBorrowed<'a> {
    //!     type Output = SomethingBorrowed<'a>;
    //!     type Error = Error;
    //!
    //!    /// Decode object from provided buffer, returning object and decoded
    //!    /// length on success
    //!    fn decode(buff: &'a [u8]) -> Result<(Self::Output, usize), Self::Error> {
    //!         let mut index = 0;
    //!
    //!         let a = buff[0];
    //!         index += 1;
    //!
    //!         // using `l` as the length of `c`
    //!         let l = buff[1] as u16 | (buff[2] as u16) << 8;
    //!         index += 2;
    //!
    //!         let c = &buff[index..][..l as usize];
    //!
    //!         Ok((Self{a, l, c}, index))
    //!    }
    //! }
    //! ```

    pub use encdec_base::decode::*;

    pub use crate::derive::{Decode, DecodeOwned};
}

// Re-export macros
pub mod derive {
    //! Macros for deriving primitive [`Decode`] and [`Encode`] implementations on types containing decodable/encodable fields.
    //!
    //! These derivations are _intended_ to be stable, **however**, if you're using these macros to implement a specific protocol you _really should_ write tests to ensure the binary encoding matches your expectations.
    //!
    //! ## Example
    //!
    //! Derive macros provide a simple way to implement [`Encode`] and [`Decode`] for objects with (enc|dec)odable fields by (de)serializing each field in order.
    //! For detail on derived implementations see the [Encode][encdec_macros::Encode] and [Decode][encdec_macros::Decode] macros.
    //!
    //! ```
    //! # use encdec::{Encode, Decode, Error};
    //! #[derive(Debug, PartialEq, Encode, Decode)]
    //! struct Something {
    //!     a: u8,
    //!     b: u16,
    //!     c: u8,
    //! }
    //! # let mut buff = [0u8; 16];
    //!
    //! let a1 = Something{ a: 0x10, b: 0xabcd, c: 0x11 };
    //! let n1 = a1.encode(&mut buff[..]).unwrap();
    //!
    //! // Encoded data is little endian, ordered by struct field
    //! assert_eq!(&buff[..n1], &[0x10, 0xcd, 0xab, 0x11]);
    //!
    //! let (a2, n2) = Something::decode(&buff[..n1]).unwrap();
    //! assert_eq!(a1, a2);
    //! ```
    //!
    //!
    //! ## Customisation
    //!
    //! ### Error Types
    //!
    //! Derived error types may be overridden with a struct level attribute
    //! `#[encdec(error = "E")]` where `E` is a user error type implementing
    //! `From<encdec::Error>`
    //!
    //! ### Encode/Decode methods
    //!
    //! Field encode/decode methods may be overridden using a field level attribute
    //! `#[encdec(with = "M")]` on a field of type `T`, where `M` is a module providing:
    //! - `fn enc(&T, &mut [u8]) -> Result<usize, E>`
    //! - `fn enc_len(&T) -> Result<usize, E>`
    //! - `fn dec(&[u8]) -> Result<(T, usize), E>`
    //!
    //!
    //! Individual methods may be overridden if required using `#[encdec(enc = "..", enc_len = "..", dec = "..")]` with the same type signatures / constraints as above.

    pub use encdec_macros::{Decode, DecodeOwned, Encode};
}

// Re-export helpers
pub mod helpers {
    //! Helpers for testing encode/decode objects as well as specialised encode/decode impls
    pub use encdec_base::helpers::*;
}
