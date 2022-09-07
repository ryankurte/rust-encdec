
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Buffer overrun in encode or decode
    BufferOverrun,
}
