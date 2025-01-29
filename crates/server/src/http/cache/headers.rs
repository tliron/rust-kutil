use http::{HeaderMap, HeaderName};

/// Header for whether to cache the response.
pub const CACHE_HEADER: HeaderName = HeaderName::from_static("x-cache");

/// Get X-Cache header. Defaults to true.
pub fn get_cache_header(headers: &HeaderMap) -> bool {
    match headers.get(CACHE_HEADER) {
        Some(value) => value == "true",
        None => true,
    }
}
