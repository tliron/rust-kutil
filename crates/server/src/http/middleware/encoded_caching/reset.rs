use super::super::super::{cache::*, errors::*};

use axum::{body::*, extract::State, http::StatusCode, response::*};

/// Handle a request to reset the cache.
pub async fn reset_cache(State(cache): State<CacheRef>) -> Result<Response, StatusCode> {
    tracing::info!("resetting cache");
    cache.invalidate_all();
    Response::builder().header(CACHE_HEADER, "false").body(Body::empty()).map_err_internal_server("build response")
}
