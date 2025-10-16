#[cfg(feature = "blocking")]
mod blocking;

#[cfg(feature = "blocking")]
#[allow(unused_imports)]
pub use blocking::*;

/// Utilities for [Stream](futures::Stream) of [Bytes](crate::std::immutable::Bytes).
#[allow(unused_imports)]
pub mod bytes;
