
/// Simple encode/decode error type
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Buffer overrun in encode or decode
    BufferOverrun,
    /// Invalid UTF8 in string
    Utf8Error,
}
