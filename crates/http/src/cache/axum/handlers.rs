use super::{super::super::cache::*, headers::*};

use ::axum::{extract::*, http::*, response::Response};

/// Axum request handler that resets the cache and returns [no_content].
///
/// Expects the state to be available for extraction. See
/// [Router::with_state](::axum::Router::with_state).
pub async fn reset_cache<CacheT, CacheKeyT>(State(cache): State<CacheT>) -> Response
where
    CacheT: Cache<CacheKeyT>,
    CacheKeyT: CacheKey,
{
    tracing::info!("resetting cache");
    cache.invalidate_all().await;
    no_content().await
}

/// Axum request handler with no content, no encoding, and no caching.
pub async fn no_content() -> Response {
    StatusCode::NO_CONTENT.do_not_encode().do_not_cache()
}
