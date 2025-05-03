use super::super::super::{cache::*, key::*, response::*};

use std::{ops::*, sync::*};

//
// MokaCacheImplementation
//

/// Moka cache implementation.
///
/// Note that it is based on the `sync` version of Moka cache rather than its `future` version.
///
/// The reason is that our `get` is not `async`, and unfortunately the `future` version of Moka
/// does not have a non-`async` version of its `get`.
pub type MokaCacheImplementation<CacheKeyT = CommonCacheKey> = Arc<moka::sync::Cache<CacheKeyT, CachedResponseRef>>;

impl<CacheKeyT> Cache<CacheKeyT> for MokaCacheImplementation<CacheKeyT>
where
    CacheKeyT: CacheKey,
{
    fn get(&self, key: &CacheKeyT) -> Option<CachedResponseRef> {
        self.deref().get(key)
    }

    async fn put(&self, key: CacheKeyT, cached_response: CachedResponseRef) {
        self.deref().insert(key, cached_response)
    }

    async fn invalidate(&self, key: &CacheKeyT) {
        self.deref().invalidate(key)
    }

    async fn invalidate_all(&self) {
        self.deref().invalidate_all()
    }
}
