// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities for transcoding.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

mod bytes;
mod encoding;

/// Utilities for transcoding while reading.
pub mod reader;

/// Utilities for transcoding.
pub mod transcode;

#[allow(unused_imports)]
pub use encoding::*;
