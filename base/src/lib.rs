//! `encdec` crate base traits

#![feature(negative_impls)]
#![feature(generic_associated_types)]
#![feature(array_try_from_fn)]

#![no_std]

mod encode;
pub use encode::*;

mod decode;
pub use decode::*;

mod error;
pub use error::Error;

mod primitives;

pub mod helpers;

pub trait EncDec<'a>: Encode + Decode<'a, Output=Self> {}

impl <'a, T: Encode + Decode<'a, Output=Self>> EncDec<'a> for T {}
