//! `encdec` crate base traits

#![feature(negative_impls)]

#![no_std]

mod encode;
pub use encode::*;

mod decode;
pub use decode::*;

mod error;
pub use error::Error;

mod primitives;

pub mod helpers;
