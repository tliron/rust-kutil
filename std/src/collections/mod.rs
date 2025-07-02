#[cfg(feature = "fast_collections")]
mod hashmaps;
#[cfg(feature = "fast_collections")]
mod hashsets;

#[cfg(feature = "fast_collections")]
#[allow(unused_imports)]
pub use {hashmaps::*, hashsets::*};
