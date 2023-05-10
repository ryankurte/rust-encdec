//! `encdec` base traits
//!

// Support initialising fixed size arrays, requires nightly
// TODO: remove when [rust-lang:89379](https://github.com/rust-lang/rust/issues/89379)
#![cfg_attr(feature = "nightly", feature(array_try_from_fn))]
#![cfg_attr(feature = "nightly", feature(associated_type_defaults))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod encode;
use encode::Encode;

pub mod decode;
use decode::{Decode, DecodeOwned};

mod error;
pub use error::Error;

pub mod primitives;

pub mod helpers;

/// Composite trait requiring an object is reversibly encodable and decodable
/// into borrowed types, useful for simplifying type bounds / generics.
///
/// (ie. `Self == <Self as Decode>::Output`, `<Self as Encode::Error> == <Self as Decode::Error>`)
pub trait EncDec<'a, E = Error>: Encode<Error = E> + Decode<'a, Output = Self, Error = E> {}

/// Automatic implementation for types implementing [`Encode`] and [`Decode`]
impl<'a, T: Encode<Error = E> + Decode<'a, Output = Self, Error = E>, E> EncDec<'a, E> for T {}

/// Composite trait requiring an object is reversibly encodable and decodable
/// into owned types, useful for simplifying type bounds / generics.
///
/// (ie. `Self == <Self as Decode>::Output`, `<Self as Encode::Error> == <Self as DecodeOwned::Error>`)
pub trait EncDecOwned<E = Error>:
    Encode<Error = E> + DecodeOwned<Output = Self, Error = E>
{
}

/// Automatic implementation for types implementing [`Encode`] and [`Decode`]
impl<T: Encode<Error = E> + DecodeOwned<Output = Self, Error = E>, E> EncDecOwned<E> for T {}
