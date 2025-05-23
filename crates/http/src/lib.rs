// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities for HTTP.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

mod body;
mod errors;
mod headers;
mod pieces;
mod uri;

/// Axum utilities.
#[cfg(feature = "axum")]
pub mod axum;

/// Cache utilities.
pub mod cache;

/// File utilities.
#[cfg(feature = "file")]
pub mod file;

/// TLS utilities.
#[cfg(feature = "tls")]
pub mod tls;

/// Tower utilities.
#[cfg(feature = "tower")]
pub mod tower;

/// Transcoding utilities.
pub mod transcoding;

#[allow(unused_imports)]
pub use {body::*, errors::*, headers::*, pieces::*, uri::*};
