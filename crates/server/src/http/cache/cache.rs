use super::{entry::*, key::*};

use std::sync::*;

//
// Cache
//

/// Cache.
pub type Cache = moka::sync::Cache<CacheKey, CacheEntry>;

/// Common reference type for [Cache].
pub type CacheRef = Arc<Cache>;
