// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
Various Rust utilities for CLI programs.

Part of the Kutil family of Rust utility libraries.

The word "kutil" means "do-it-yourselfer" in Czech.

For more information and usage examples see the
[home page](https://github.com/tliron/rust-kutil).
*/

/// Clap utilities.
pub mod clap;

/// Debugging utilities.
pub mod debug;

/// Logging and tracing utilities.
pub mod log;

/// Main loop utilities.
pub mod run;
