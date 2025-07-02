/// Fast [HashSet](std::collections::HashSet).
///
/// The implementation uses [ahash::RandomState].
pub type FastHashSet<ValueT> = std::collections::HashSet<ValueT, ahash::RandomState>;

pub use ahash::HashSetExt;
