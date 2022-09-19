//! `encdec` base traits
//! 

#![feature(negative_impls)]
#![feature(generic_associated_types)]
#![feature(array_try_from_fn)]

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod encode;
use encode::Encode;

pub mod decode;
use decode::Decode;

mod error;
pub use error::Error;

pub mod primitives;

pub mod helpers;

/// Composite trait requiring an object is both encodable and decodable
pub trait EncDec<'a>: Encode + Decode<'a, Output=Self> {}

/// Automatic implementation for types implementing [`Encode`] and [`Decode`]
impl <'a, T: Encode + Decode<'a, Output=Self>> EncDec<'a> for T {}
