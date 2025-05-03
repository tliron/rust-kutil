#[cfg(feature = "blocking")]
mod blocking;

#[cfg(feature = "blocking")]
#[allow(unused_imports)]
pub use blocking::*;

/// Utilities for [Stream](futures::Stream) of [Bytes](::bytes::Bytes).
#[allow(unused_imports)]
pub mod bytes;
