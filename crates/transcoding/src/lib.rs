#![warn(missing_docs)]

/*!
Various Rust utilities for transcoding.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

mod encoding;

/// Utilities for transcoding bytes.
pub mod bytes;

/// Utilities for transcoding while reading.
pub mod reader;

#[allow(unused_imports)]
pub use encoding::*;
