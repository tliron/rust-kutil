//
// Sized
//

/// Sized.
pub trait Sized {
    /// Size in bytes.
    ///
    /// This is *not* the exact amount of memory we are using, but rather an indictator that can
    /// be useful for comparisons.
    fn size(&self) -> usize;
}
