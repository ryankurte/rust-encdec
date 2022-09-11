//! Simple object encoding / decoding helpers
//! 
//! This is intend to provide a straightforward method of serialising objects 
//! similar to (but not explicitly compatible with) C's packed representation.

#![feature(generic_associated_types)]

#![no_std]

pub use encdec_base::{Encode, Decode, Error};

pub use encdec_macros::{Encode, Decode};

pub mod helpers {
    pub use encdec_base::helpers::*;
}
