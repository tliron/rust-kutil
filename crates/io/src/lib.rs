#![warn(missing_docs)]

/*!
Various Rust utilities for I/O.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

/// Utilities for [Stream](futures::Stream) of [Bytes](bytes::Bytes).
pub mod bytes_stream;

/// Utilities for [io::Read](std::io::Read).
pub mod reader;

/// Utilities for [Stream](futures::Stream).
pub mod stream;
