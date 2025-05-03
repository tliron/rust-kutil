use super::{key::*, response::*};

//
// Cache
//

/// Cache.
///
/// Cloning should be cheap! Wrapping an implementation [Arc](std::sync::Arc) might be an easy
/// solution.
#[allow(async_fn_in_trait)]
pub trait Cache<CacheKeyT = CommonCacheKey>: 'static + Clone + Send + Sync
where
    CacheKeyT: CacheKey,
{
    /// Get an entry from the cache.
    ///
    /// Note that `get` is *not* `async`. The reason is that we need the result in the context of a
    /// polling function that is itself not `async`.
    fn get(&self, key: &CacheKeyT) -> Option<CachedResponseRef>;

    /// Put an entry in the cache.
    ///
    /// The cache should take into consideration the [CachedResponse::duration] if set.
    ///
    /// Note that this is an `async` function written in longer form in order to include the `Send`
    /// constraint. Implementations can simply use `async fn put`.
    fn put(&self, key: CacheKeyT, cached_response: CachedResponseRef) -> impl Future<Output = ()> + Send;

    /// Invalidate a cache entry.
    async fn invalidate(&self, key: &CacheKeyT);

    /// Invalidate all cache entries.
    async fn invalidate_all(&self);
}
