//! Basic encdec [`Error`] type

/// Basic encode/decode error type
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
pub enum Error {
    /// Buffer length error in encode or decode
    Length,
    /// Invalid UTF8 in string
    Utf8,
}
