// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities to enhance the standard library.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

/// [Any](std::any::Any) utilities.
pub mod any;

/// Borrow utilities.
pub mod borrow;

/// Collections utilities.
pub mod collections;

/// Error utilities.
pub mod error;

/// Foster utilities.
pub mod foster;

/// Filesystem utilities.
pub mod fs;

/// Future utilities.
pub mod future;

/// Iteration utilities.
pub mod iter;

/// Metric utilities.
pub mod metric;

/// String utilities.
pub mod string;

/// Synchronization utilities.
pub mod sync;

/// Zero-copy utilities.
#[cfg(feature = "zerocopy")]
#[allow(unused_imports)]
pub mod zerocopy;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use kutil_std_macros::*;
