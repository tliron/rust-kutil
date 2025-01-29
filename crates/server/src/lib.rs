#![warn(missing_docs)]

/*!
Various Rust utilities for servers.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

/// Bind utilities.
pub mod bind;

/// HTTP utilities.
#[cfg(feature = "http")]
pub mod http;
