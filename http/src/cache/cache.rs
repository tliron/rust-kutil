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
    /// Note that this is an `async` function written in longer form in order to include the `Send`
    /// constraint. Implementations can simply use `async fn put`.
    fn get(&self, key: &CacheKeyT) -> impl Future<Output = Option<CachedResponseRef>> + Send;

    /// Put an entry in the cache.
    ///
    /// The cache should take into consideration the [CachedResponse::duration] if set.
    ///
    /// Note that this is an `async` function written in longer form in order to include the `Send`
    /// constraint. Implementations can simply use `async fn put`.
    fn put(&self, key: CacheKeyT, cached_response: CachedResponseRef) -> impl Future<Output = ()> + Send;

    /// Invalidate a cache entry.
    ///
    /// Note that this is an `async` function written in longer form in order to include the `Send`
    /// constraint. Implementations can simply use `async fn invalidate`.
    fn invalidate(&self, key: &CacheKeyT) -> impl Future<Output = ()> + Send;

    /// Invalidate all cache entries.
    ///
    /// Note that this is an `async` function written in longer form in order to include the `Send`
    /// constraint. Implementations can simply use `async fn invalidate_all`.
    fn invalidate_all(&self) -> impl Future<Output = ()> + Send;
}
