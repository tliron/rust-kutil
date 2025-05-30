/// Fast [HashMap](std::collections::HashMap).
///
/// The implementation uses [ahash::RandomState].
pub type FastHashMap<KeyT, ValueT> = std::collections::HashMap<KeyT, ValueT, ahash::RandomState>;

/// Fast concurrent [HashMap](std::collections::HashMap).
///
/// The implementation uses [papaya::HashMap] and [ahash::RandomState].
pub type FastConcurrentHashMap<KeyT, ValueT> = papaya::HashMap<KeyT, ValueT, ahash::RandomState>;

pub use ahash::HashMapExt;
