# encdec

Helpers for simple binary object encoding and decoding, for when you need to support a specific C-object-like protocol without manually writing encode/decode everywhere...
This provides `Encode` and `Decode` traits for relevant types, as well as derive macros that generate sequential encode/decode implementations for objects with encodable fields.

