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

/// Foster utilities.
pub mod foster;

/// Borrow utilities.
pub mod borrow;

/// Collections utilities.
pub mod collections;

/// Error utilities.
pub mod error;

/// Future utilities.
pub mod future;

/// Iteration utilities.
pub mod iter;

/// String utilities.
pub mod string;

/// Synchronization utilities.
pub mod sync;
