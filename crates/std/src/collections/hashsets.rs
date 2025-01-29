/// Fast hashset.
///
/// The implementation uses [ahash::RandomState].
pub type FastHashSet<ValueT> = std::collections::HashMap<ValueT, ahash::RandomState>;

pub use ahash::HashSetExt;
