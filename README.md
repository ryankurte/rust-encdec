# encdec
 
This crate provides common (and `no_std` compatible) [`Encode`] and [`Decode`]  traits for describing binary encode/decode-able objects in embedded contexts, as well as derive macros to automagically implement these over objects, and basic (at this time little-endian only) implementations for primitive types.

This is intended for use where you need to binary encode objects to suit a particular protocol or specification, because who hasn't had enough of creating per-project encoding traits, and for everything else there are neater solutions like [prost](https://crates.io/crates/prost) for protobufs or [serde](https://crates.io/crates/serde) and [postcard](https://crates.io/crates/postcard) if all consumers are using rust.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-encdec.svg)](https://github.com/ryankurte/rust-encdec)
![Build Status](https://github.com/ryankurte/rust-encdec/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/encdec.svg)](https://crates.io/crates/encdec)
[![Docs.rs](https://docs.rs/encdec/badge.svg)](https://docs.rs/encdec)

