//! Basic encdec [`Error`] type

/// Basic encode/decode error type
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
pub enum Error {
    /// Buffer overrun in encode or decode
    BufferOverrun,
    /// Invalid UTF8 in string
    Utf8Error,
}
