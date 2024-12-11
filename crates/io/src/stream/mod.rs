#[cfg(feature = "blocking")]
mod blocking;

#[cfg(feature = "blocking")]
#[allow(unused_imports)]
pub use blocking::*;
