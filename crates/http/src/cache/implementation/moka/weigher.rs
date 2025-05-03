use super::super::super::{key::*, response::*, sized::*};

/// Moka cache entry weigher.
pub fn weigher<CacheKeyT>(cache_key: &CacheKeyT, cached_response: &CachedResponseRef) -> u32
where
    CacheKeyT: CacheKey,
{
    let weight = cache_key.size() + cached_response.size();
    let weight = weight.try_into().unwrap_or(u32::MAX);
    tracing::debug!("{} for {}", weight, cache_key);
    weight
}
